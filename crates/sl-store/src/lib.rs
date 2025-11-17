use serde::{Deserialize, Serialize};
use sl_utils::{errors::BackendError, requester::Requester};

use crate::{
    curseforge::api::{search::{get_curseforge_search, CurseforgeSearchResponse}, CurseforgeProjectVersion}, modrinth::api::{project::get_modrinth_project_versions, search::get_modrinth_search, ModrinthProject, ModrinthSearchResult},
};

pub mod curseforge;
pub mod modrinth;

pub(crate) const PAGE_SIZE: u32 = 16;

const MODRINTH_MODPACK_PROJECT_TYPE: &str = "modpack";
const MODRINTH_MOD_PROJECT_TYPE: &str = "mod";
const MODRINTH_SHADERPACK_PROJECT_TYPE: &str = "shader";
const MODRINTH_RESOURCEPACK_PROJECT_TYPE: &str = "resourcepack";

const CURSEFORGE_MODPACK_CLASS_ID: u32 = 4471;
const CURSEFORGE_MOD_CLASS_ID: u32 = 6;
const CURSEFORGE_SHADERPACK_CLASS_ID: u32 = 6552;
const CURSEFORGE_RESOURCEPACK_CLASS_ID: u32 = 12;

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
    Curseforge(Vec<CurseforgeProjectVersion>)
}

pub async fn get_store_search(
    store_type: StoreType,
    store_category: StoreCategory,
    search_query: &str,
    store_page: u32,
    requester: &Requester
) -> Result<StoreSearchResult, BackendError> {
    match store_type {
        StoreType::Modrinth => {
            let search_result = match store_category {
                StoreCategory::Modpacks => get_modrinth_search(requester, search_query, MODRINTH_MODPACK_PROJECT_TYPE, store_page).await,
                StoreCategory::Mods => get_modrinth_search(requester, search_query, MODRINTH_MOD_PROJECT_TYPE, store_page).await,
                StoreCategory::Shaderpacks => get_modrinth_search(requester, search_query, MODRINTH_SHADERPACK_PROJECT_TYPE, store_page).await,
                StoreCategory::Resourcepacks => get_modrinth_search(requester, search_query, MODRINTH_RESOURCEPACK_PROJECT_TYPE, store_page).await
            }?;

            Ok(StoreSearchResult::Modrinth(search_result))
        }
        StoreType::Curseforge => {
            let search_result = match store_category {
                StoreCategory::Modpacks => get_curseforge_search(requester, search_query, CURSEFORGE_MODPACK_CLASS_ID, store_page).await,
                StoreCategory::Mods => get_curseforge_search(requester, search_query, CURSEFORGE_MOD_CLASS_ID, store_page).await,
                StoreCategory::Shaderpacks => get_curseforge_search(requester, search_query, CURSEFORGE_RESOURCEPACK_CLASS_ID, store_page).await,
                StoreCategory::Resourcepacks => get_curseforge_search(&requester, search_query, CURSEFORGE_SHADERPACK_CLASS_ID, store_page).await
            }?;

            Ok(StoreSearchResult::Curseforge(search_result))
        }
    }
}

pub async fn get_content_versions(
    store_type: StoreType,
    slug: &str,
    game_version: Option<&str>,
    loader: Option<&str>,
    requester: &Requester
) -> Result<StoreProjectVersions, BackendError>
{
    match store_type {
        StoreType::Modrinth => {
            let versions = get_modrinth_project_versions(
                requester, 
                slug, 
                game_version, 
                loader
            ).await?;

            Ok(StoreProjectVersions::Modrinth(versions))
        }
        StoreType::Curseforge => {
            todo!()
        }
    }
}