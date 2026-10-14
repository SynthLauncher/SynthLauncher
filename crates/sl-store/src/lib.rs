use serde::{Deserialize, Serialize};
use sl_utils::{errors::HttpError, requester::Requester};

use crate::modrinth::api::{search::{search_modrinth_store, ModrinthSearchHit, ModrinthSearchHits}, ModrinthProjectFile, ProjectType};

pub mod curseforge;
pub mod modrinth;

pub(crate) const PAGE_SIZE: u32 = 16;

#[derive(Debug, Deserialize, Serialize)]
pub enum ContentIdentifier {
    Modrinth(String),
    Curseforge(u32),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StoreSource {
    Modrinth,
    Curseforge,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StoreContentCategory {
    Modpacks,
    Mods,
    Resourcepacks,
    Shaderpacks,
}

impl From<StoreContentCategory> for &'static str {
    fn from(value: StoreContentCategory) -> Self {
        match value {
            StoreContentCategory::Modpacks => "modpack",
            StoreContentCategory::Mods => "mod",
            StoreContentCategory::Resourcepacks => "resourcepack",
            StoreContentCategory::Shaderpacks => "shader",
        }
    }
}

impl From<ProjectType> for StoreContentCategory {
    fn from(value: ProjectType) -> Self {
        match value {
            ProjectType::Mod => StoreContentCategory::Mods,
            ProjectType::Modpack => StoreContentCategory::Modpacks,
            ProjectType::Resourcepack => StoreContentCategory::Resourcepacks,
            ProjectType::Shader => StoreContentCategory::Shaderpacks,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StoreContentSearchHit {
    source: StoreSource,
    r#type: StoreContentCategory,
    id: ContentIdentifier,
    name: String,
    slug: String,
    description: String,
    icon_url: Option<String>,
    author: String,
    downloads: u64,
}

impl From<ModrinthSearchHit> for StoreContentSearchHit {
    fn from(value: ModrinthSearchHit) -> Self {
        Self {
            source: StoreSource::Modrinth,
            id: ContentIdentifier::Modrinth(value.project_id),
            name: value.title,
            slug: value.slug,
            description: value.description,
            icon_url: value.icon_url,
            author: value.author,
            downloads: value.downloads as u64,
            r#type: value.project_type.into(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StoreContentSearchHits {
    pub hits: Vec<StoreContentSearchHit>,
    pub total_hits: u32,
}

impl StoreContentSearchHits {
    pub fn new() -> Self {
        Self {
            hits: Vec::with_capacity(PAGE_SIZE as usize),
            total_hits: 0,
        }
    }
}

impl From<ModrinthSearchHits> for StoreContentSearchHits {
    fn from(value: ModrinthSearchHits) -> Self {
        let mut content_hits = StoreContentSearchHits::new();

        content_hits.total_hits = value.total_hits;
        for hit in value.hits {
            content_hits.hits.push(hit.into());
        }

        content_hits
    }
}

pub async fn search_store(
    requester: &Requester,
    query: &str,
    r#type: StoreContentCategory,
    offset: u32,
    source: StoreSource,
) -> Result<StoreContentSearchHits, HttpError> {
    match source {
        StoreSource::Modrinth => {
            let hits = search_modrinth_store(&requester, query, r#type.into(), offset).await?;

            Ok(hits.into())
        }
        StoreSource::Curseforge => todo!(),
    }
}

#[derive(Debug)]
pub struct StoreContentVersion
{
    id: ContentIdentifier,
    project_id: String,
    game_versions: Vec<String>,
    loaders: Vec<String>,

} 