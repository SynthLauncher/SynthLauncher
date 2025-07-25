use std::{
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sl_core::REQUESTER;
use sl_utils::errors::BackendError;

use crate::modrinth::api::{
    GalleryImage, ProjectType,
};

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
    pub gallery: Vec<GalleryImage>,
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
    pub version_type: String,
    pub files: Vec<ModrinthProjectFile>,
}

#[must_use]
pub async fn query_project(slug: &str) -> Result<ModrinthProject, BackendError> {
    let url = format!("https://api.modrinth.com/v2/project/{}", slug);
    let json = REQUESTER.get_json(&url).await?;
    Ok(json)
}

#[must_use]
pub async fn query_project_versions(slug: &str, game_version: &str, loader: &str) -> Result<Vec<ModrinthProjectVersion>, BackendError> {
    let url = format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]&loaders=[\"{}\"]", slug, game_version, loader);
    let json = REQUESTER.get_json(&url).await?;
    Ok(json)
}

#[must_use]
pub async fn query_project_version(
    slug: &str,
    version: &str,
) -> Result<ModrinthProjectVersion, BackendError> {
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version/{}",
        slug, version
    );
    let json = REQUESTER.get_json(&url).await?;
    Ok(json)
}

pub async fn download_project_file(
    project_file: &ModrinthProjectFile,
    dest: &Path,
) -> Result<PathBuf, BackendError> {
    let path = dest.join(&project_file.filename);
    
    REQUESTER
        .builder()
        .download_to(&project_file.url, &path)
        .await?;

    Ok(path)
}
