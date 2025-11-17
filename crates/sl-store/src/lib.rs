use serde::{Deserialize, Serialize};
use sl_core::instances::InstanceManager;
use sl_utils::{errors::BackendError, requester::Requester};

use crate::{
    curseforge::api::{
        search::{get_curseforge_search, CurseforgeSearchResponse},
        CurseforgeProjectVersion,
    },
    modrinth::api::{
        project::get_modrinth_project_versions, search::get_modrinth_search, ModrinthProject,
        ModrinthProjectFile, ModrinthSearchResult,
    },
};

pub mod curseforge;
pub mod modrinth;

pub(crate) const PAGE_SIZE: u32 = 16;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StoreType {
    Modrinth,
    Curseforge,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StoreCategory {
    Modpacks,
    Mods,
    Shaderpacks,
    Resourcepacks,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StoreSearchResult {
    Modrinth(ModrinthSearchResult),
    Curseforge(CurseforgeSearchResponse),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StoreProjectVersions {
    Modrinth(Vec<ModrinthProject>),
    Curseforge(Vec<CurseforgeProjectVersion>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContentFile {
    Modrinth(ModrinthProjectFile),
}

pub async fn get_store_search(
    store_type: StoreType,
    store_category: StoreCategory,
    search_query: &str,
    store_page: u32,
    requester: &Requester,
) -> Result<StoreSearchResult, BackendError> {
    match store_type {
        StoreType::Modrinth => {
            let search_result = get_modrinth_search(
                        requester,
                        search_query,
                        store_category.into(),
                        store_page,
                    )
                    .await?;

            Ok(StoreSearchResult::Modrinth(search_result))
        }
        StoreType::Curseforge => {
            let search_result = 
                    get_curseforge_search(
                        requester,
                        search_query,
                        store_category.into(),
                        store_page,
                    )
                    .await?;

            Ok(StoreSearchResult::Curseforge(search_result))
        }
    }
}

pub async fn get_content_versions(
    store_type: StoreType,
    slug: &str,
    game_version: Option<&str>,
    loader: Option<&str>,
    requester: &Requester,
) -> Result<StoreProjectVersions, BackendError> {
    match store_type {
        StoreType::Modrinth => {
            let versions =
                get_modrinth_project_versions(requester, slug, game_version, loader).await?;

            Ok(StoreProjectVersions::Modrinth(versions))
        }
        StoreType::Curseforge => {
            todo!()
        }
    }
}

pub async fn download_content<'a>(
    requester: &Requester,
    instance_manager: &InstanceManager<'a>,
    instance_name: &str,
    files: Vec<ContentFile>,
) -> Result<(), BackendError> {
    for file in files {
        match file {
            ContentFile::Modrinth(file) => {
                let path = instance_manager
                    .instance_dir(instance_name)
                    .join("mods")
                    .join(file.filename());
                if let Some(parent) = path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                requester.builder().download_to(&file.url(), &path).await?;
            }
        }
    }

    Ok(())
}

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
