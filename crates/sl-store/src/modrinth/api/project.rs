use std::{fmt::Write, path::Path, sync::Arc};

use dashmap::DashMap;
use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use sl_utils::{errors::BackendError, requester::Requester};

use crate::modrinth::api::ProjectType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProjectDependency {
    pub project_id: String,
    pub dependency_type: String, // TODO: Maybe make this an enum
    pub version_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProject {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub project_type: ProjectType,
    pub downloads: u32,
    pub icon_url: Option<String>,
    pub followers: u32,
    /// Project versions
    pub versions: Vec<String>,
    /// Supported Minecraft versions
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

type ProjectId = String;
type VersionId = String;

#[derive(Debug, Clone)]
pub struct ResolutionState {
    pub resolved: Arc<DashMap<ProjectId, VersionId>>,
}

impl ResolutionState {
    pub fn new() -> Self {
        Self {
            resolved: Arc::new(DashMap::new()),
        }
    }
}

async fn pick_latest_version_id(
    requester: &Requester,
    project_id: &str,
    game_version: &str,
    loader: &str,
) -> Result<String, BackendError> {
    let versions =
        query_project_versions(requester, project_id, Some(game_version), Some(loader)).await?;

    let latest_id = versions[0].id.clone();
    Ok(latest_id)
}

pub async fn resolve_mod(
    requester: &Requester,
    state: &ResolutionState,
    project_id: String,
    version_id: String,
    game_version: &str,
    loader: &str,
) -> Result<(), BackendError> {
    if state.resolved.contains_key(&project_id) {
        return Ok(());
    }

    state
        .resolved
        .insert(project_id.clone(), version_id.clone());

    let version_info = query_project_version(requester, &project_id, &version_id).await?;

    let mut futures = FuturesUnordered::new();

    for dep in &version_info.dependencies {
        if dep.dependency_type != "required" {
            continue;
        }

        let dep_vid = if let Some(vid) = &dep.version_id {
            vid.clone()
        } else {
            pick_latest_version_id(requester, &dep.project_id, game_version, loader).await?
        };

        futures.push(resolve_mod(
            requester,
            state,
            dep.project_id.clone(),
            dep_vid,
            game_version,
            loader,
        ));
    }

    while let Some(res) = futures.next().await {
        res?;
    }

    Ok(())
}

#[must_use]
pub async fn query_project(
    requester: &Requester,
    slug: &str,
) -> Result<ModrinthProject, BackendError> {
    let url = format!("https://api.modrinth.com/v2/project/{}", slug);
    let json = requester.get_json(&url).await?;
    Ok(json)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProjectFileHashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProjectFile {
    pub hashes: ModrinthProjectFileHashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u32,
    pub file_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProjectVersion {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub version_number: String,
    pub downloads: u32,
    pub files: Vec<ModrinthProjectFile>,
    pub dependencies: Vec<ModrinthProjectDependency>,
}

#[must_use]
pub async fn query_project_versions(
    requester: &Requester,
    slug: &str,
    game_version: Option<&str>,
    loader: Option<&str>,
) -> Result<Vec<ModrinthProjectVersion>, BackendError> {
    let mut url = format!("https://api.modrinth.com/v2/project/{}/version?", slug);

    let mut query_params = Vec::new();

    if let Some(game_version) = game_version {
        query_params.push(format!("game_versions=[\"{}\"]", game_version));
    }

    if let Some(loader) = loader {
        query_params.push(format!("loaders=[\"{}\"]", loader));
    }

    _ = write!(url, "{}", query_params.join("&"));

    let json = requester.get_json(&url).await?;
    Ok(json)
}

#[must_use]
pub async fn query_project_version(
    requester: &Requester,
    slug: &str,
    version: &str,
) -> Result<ModrinthProjectVersion, BackendError> {
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version/{}",
        slug, version
    );
    let json = requester.get_json(&url).await?;
    Ok(json)
}

pub async fn download_project_file(
    requester: &Requester,
    project_file: &ModrinthProjectFile,
    dest: &Path,
) -> Result<(), BackendError> {
    requester
        .builder()
        .download_to(&project_file.url, &dest.join(&project_file.filename))
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_resolve_mod_real_api_latest() {
        let state = ResolutionState::new();

        let project_id = "modmenu".to_string();
        let requester = Requester::new();
        let latest_versions = query_project_versions(&requester, &project_id, None, None)
            .await
            .expect("Failed to fetch project versions");

        assert!(
            !latest_versions.is_empty(),
            "No versions found for project {}",
            project_id
        );

        let latest_version = &latest_versions[0];

        println!("{:?}", latest_version.dependencies);

        let result = resolve_mod(
            &requester,
            &state.clone(),
            project_id.clone(),
            latest_version.id.clone(),
            "1.20.1",
            "fabric",
        )
        .await;

        assert!(result.is_ok(), "resolve_mod failed: {:?}", result.err());

        assert!(
            state.resolved.contains_key(&project_id),
            "Expected {} to be in resolved map",
            project_id
        );

        println!("Resolved mods: {:#?}", state.resolved);
    }
}
