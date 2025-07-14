use std::{
    path::{Path, PathBuf},
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sl_utils::errors::BackendError;
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::modrinth::api::{
    GalleryImage, ProjectType,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub project_type: ProjectType,
    pub downloads: u32,
    pub icon_url: Option<String>,
    pub id: String,
    pub team: String,
    pub body_url: Option<String>,
    pub followers: u32,
    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub gallery: Vec<GalleryImage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u32,
    pub file_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectVersion {
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub name: String,
    pub version_number: String,
    pub downloads: u32,
    pub version_type: String,
    pub files: Vec<ProjectFile>,
}

#[must_use]
pub async fn query_project(client: &Client, slug: &str) -> Result<Project, BackendError> {
    let url = format!("https://api.modrinth.com/v2/project/{}", slug);
    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}

#[must_use]
pub async fn query_project_versions(
    client: &Client,
    slug: &str,
) -> Result<Vec<ProjectVersion>, BackendError> {
    let url = format!("https://api.modrinth.com/v2/project/{}/version", slug);
    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}

#[must_use]
pub async fn query_project_version(
    client: &Client,
    slug: &str,
    version: &str,
) -> Result<ProjectVersion, BackendError> {
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version/{}",
        slug, version
    );
    let json = client.get(url).send().await?.json().await?;
    Ok(json)
}

pub async fn download_project_file(
    client: &Client,
    project_file: &ProjectFile,
    dest: &Path,
) -> Result<PathBuf, BackendError> {
    let mut res = client.get(&project_file.url).send().await?;
    let path = dest.join(&project_file.filename);

    let file = tokio::fs::File::create(&path).await?;
    let mut writer = BufWriter::new(file);

    while let Some(chunk) = res.chunk().await? {
        writer.write_all(&chunk).await?;
    }

    Ok(path)
}
