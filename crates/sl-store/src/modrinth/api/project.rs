use std::{fmt::Write, path::Path};

use serde::{Deserialize, Serialize};
use sl_utils::{errors::BackendError, requester::Requester};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack,
    Resourcepack,
    Shader,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProjectDependency {
    pub project_id: Option<String>,
    pub version_id: Option<String>,
    pub dependency_type: String, // TODO: Maybe make this an enum
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
    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

#[must_use]
pub async fn get_modrinth_project(
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
pub async fn get_modrinth_project_versions(
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

    Ok(requester.get_json(&url).await?)
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
    Ok(requester.get_json(&url).await?)
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
