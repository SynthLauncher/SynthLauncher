use std::{
    path::{Path, PathBuf},
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sl_utils::errors::BackendError;
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::modrinth::api::{
    DonationURL, GalleryImage, License, ModeratorMessage, MonetizationStatus, ProjectType,
    RequestedStatusType, StatusType, SupportRequirement,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: SupportRequirement,
    pub server_side: SupportRequirement,
    pub body: String,
    pub status: StatusType,
    pub requested_status: Option<RequestedStatusType>,
    pub additional_categories: Vec<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Vec<DonationURL>,
    pub project_type: ProjectType,
    pub downloads: u32,
    pub icon_url: Option<String>,
    pub color: Option<i32>,
    pub thread_id: String,
    pub monetization_status: MonetizationStatus,
    pub id: String,
    pub team: String,
    pub body_url: Option<String>,
    pub moderator_message: Option<ModeratorMessage>,
    pub published: String,        // format: ISO-8601
    pub updated: String,          // format: ISO-8601
    pub approved: Option<String>, // format: ISO-8601
    pub queued: Option<String>,   // format: ISO-8601
    pub followers: u32,
    pub license: License,
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
    pub featured: bool,
    pub name: String,
    pub version_number: String,
    pub changelog: String,
    pub changelog_url: Option<String>,
    pub data_published: Option<String>,
    pub downloads: u32,
    pub version_type: String,
    pub status: StatusType,
    pub requested_status: Option<RequestedStatusType>,
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
