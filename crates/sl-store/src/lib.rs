use serde::{Deserialize, Serialize};
use sl_utils::{errors::HttpError, requester::Requester};

use crate::modrinth::api::{search::{search_modrinth_store, ModrinthSearchHit, ModrinthSearchHits}, ProjectType};

pub mod curseforge;
pub mod modrinth;

pub(crate) const PAGE_SIZE: u32 = 16;

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "lowercase")]
// pub enum StoreType {
//     Modrinth,
//     Curseforge,
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "lowercase")]
// pub enum StoreCategory {
//     Modpacks,
//     Mods,
//     Shaderpacks,
//     Resourcepacks,
// }

// impl From<StoreCategory> for &'static str {
//     fn from(value: StoreCategory) -> Self {
//         match value {
//             StoreCategory::Modpacks => "modpack",
//             StoreCategory::Mods => "mod",
//             StoreCategory::Resourcepacks => "resourcepack",
//             StoreCategory::Shaderpacks => "shader",
//         }
//     }
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
// pub enum StoreSearchResult {
//     Modrinth(ModrinthSearchHits),
//     Curseforge(CurseforgeSearchResponse),
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(tag = "source", content = "data")]
// pub enum StoreSearch {
//     Modrinth(ModrinthSearchHits),
//     Cursefore(CurseforgeSearchResponse)
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
// pub enum StoreProjectVersions {
//     Modrinth(Vec<ModrinthProject>),
//     Curseforge(Vec<CurseforgeProjectVersion>),
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
// pub enum ContentFile {
//     Modrinth(ModrinthProjectFile),
// }

// pub async fn get_store_search(
//     store_type: StoreType,
//     store_category: StoreCategory,
//     search_query: &str,
//     store_page: u32,
//     requester: &Requester,
// ) -> Result<StoreSearchResult, BackendError> {
//     match store_type {
//         StoreType::Modrinth => {
//             let search_result = search_modrinth_store(
//                         requester,
//                         search_query,
//                         store_category.into(),
//                         store_page,
//                     )
//                     .await?;

//             Ok(StoreSearchResult::Modrinth(search_result))
//         }
//         StoreType::Curseforge => {
//             let search_result =
//                     get_curseforge_search(
//                         requester,
//                         search_query,
//                         store_category.into(),
//                         store_page,
//                     )
//                     .await?;

//             Ok(StoreSearchResult::Curseforge(search_result))
//         }
//     }
// }

// pub async fn get_content_versions(
//     store_type: StoreType,
//     slug: &str,
//     game_version: Option<&str>,
//     loader: Option<&str>,
//     requester: &Requester,
// ) -> Result<StoreProjectVersions, BackendError> {
//     match store_type {
//         StoreType::Modrinth => {
//             let versions =
//                 get_modrinth_project_versions(requester, slug, game_version, loader).await?;

//             println!("{:?}", versions);
//             Ok(StoreProjectVersions::Modrinth(versions))
//         }
//         StoreType::Curseforge => {
//             todo!()
//         }
//     }
// }

// pub async fn download_content<'a>(
//     requester: &Requester,
//     instance_manager: &InstanceManager<'a>,
//     instance_name: &str,
//     files: Vec<ContentFile>,
// ) -> Result<(), BackendError> {
//     for file in files {
//         match file {
//             ContentFile::Modrinth(file) => {
//                 let path = instance_manager
//                     .instance_dir(instance_name)
//                     .join("mods")
//                     .join(file.filename());
//                 if let Some(parent) = path.parent() {
//                     tokio::fs::create_dir_all(parent).await?;
//                 }

//                 requester.builder().download_to(&file.url(), &path).await?;
//             }
//         }
//     }

//     Ok(())
// }

// pub async fn download_content<'a>(requester: &Requester, instance_manager: &InstanceManager<'a>, instance_name: &str, filename: &str, url: &str) -> Result<(), BackendError>
// {
// let path = instance_manager.instance_dir(instance_name).join("mods").join(filename);
// if let Some(parent) = path.parent() {
//     tokio::fs::create_dir_all(parent).await?;
// }

// requester
//     .builder()
//     .download_to(&url, &path)
//     .await?;

//     Ok(())
// }

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
