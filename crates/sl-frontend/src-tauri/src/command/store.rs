use serde::{Deserialize, Serialize};
use sl_core::{environment::LauncherEnv, instances::content_caching::ContentCachingManager};
use sl_store::{
    curseforge::api::{search::{get_curseforge_search, CurseforgeSearchResponse}, CurseforgeProjectVersion},
    modrinth::{
        api::{project::{get_modrinth_project_versions, ModrinthProjectVersion, ProjectType}, search::{get_modrinth_search, ModrinthSearchResult}}, download_modrinth_project,
    },
};
use tauri::State;
use tokio::sync::RwLock;

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
pub enum StoreSearch {
    Modrinth(ModrinthSearchResult),
    Curseforge(CurseforgeSearchResponse),
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
// pub enum ContentVersions {
//     Modrinth(Vec<ModrinthProjectVersion>),
//     Curseforge(Vec<CurseforgeProjectVersion>)
// }

const CURSEFORGE_MODPACK_CLASS_ID: u32 = 4471;
const CURSEFORGE_MOD_CLASS_ID: u32 = 6;
const CURSEFORGE_SHADERPACK_CLASS_ID: u32 = 6552;
const CURSEFORGE_RESOURCEPACK_CLASS_ID: u32 = 12;

const MODRINTH_MODPACK_PROJECT_TYPE: &str = "modpack";
const MODRINTH_MOD_PROJECT_TYPE: &str = "mod";
const MODRINTH_SHADERPACK_PROJECT_TYPE: &str = "shader";
const MODRINTH_RESOURCEPACK_PROJECT_TYPE: &str = "resourcepack";

#[tauri::command]
pub async fn fetch_store_search(
    search_query: &str,
    store_type: StoreType,
    store_category: StoreCategory,
    store_page: u32,
    launcher_env_state: State<'_, RwLock<LauncherEnv>>,
) -> Result<StoreSearch, String> {
    let launcher_env = launcher_env_state.read().await;
    let requester = launcher_env.requester();

    match store_type {
        StoreType::Modrinth => {
            let search_result = match store_category {
                StoreCategory::Modpacks => get_modrinth_search(requester, search_query, MODRINTH_MODPACK_PROJECT_TYPE, store_page).await,
                StoreCategory::Mods => get_modrinth_search(requester, search_query, MODRINTH_MOD_PROJECT_TYPE, store_page).await,
                StoreCategory::Shaderpacks => get_modrinth_search(requester, search_query, MODRINTH_SHADERPACK_PROJECT_TYPE, store_page).await,
                StoreCategory::Resourcepacks => get_modrinth_search(requester, search_query, MODRINTH_RESOURCEPACK_PROJECT_TYPE, store_page).await
            }
            .map_err(|e| e.to_string())?;

            Ok(StoreSearch::Modrinth(search_result))
        }
        StoreType::Curseforge => {
            let search_result = match store_category {
                StoreCategory::Modpacks => get_curseforge_search(requester, search_query, CURSEFORGE_MODPACK_CLASS_ID, store_page).await,
                StoreCategory::Mods => get_curseforge_search(requester, search_query, CURSEFORGE_MOD_CLASS_ID, store_page).await,
                StoreCategory::Shaderpacks => get_curseforge_search(requester, search_query, CURSEFORGE_RESOURCEPACK_CLASS_ID, store_page).await,
                StoreCategory::Resourcepacks => get_curseforge_search(&requester, search_query, CURSEFORGE_SHADERPACK_CLASS_ID, store_page).await
            }
            .map_err(|e| e.to_string())?;

            Ok(StoreSearch::Curseforge(search_result))
        }
    }
}

// #[tauri::command]
// pub async fn get_content_versions(
//     slug: &str,
//     store_type: StoreType,
//     store_category: StoreCategory,
//     launcher_env: State<'_, RwLock<LauncherEnv>>,
// ) -> Result<ContentVersion, String> {
//     let env = launcher_env.read().await;
//     let requester = env.requester();
    
//     match store_type {
//         StoreType::Modrinth => {
//             let versions = match store_category {
//                 StoreCategory::Modpacks => get_projects_versions(requester, slug, None, None, ProjectType::Modpack),
//                 StoreCategory::Mods => get_projects_versions(requester, slug, None, None, ProjectType::Mod),
//                 StoreCategory::Resourcepacks => get_projects_versions(requester, slug, None, None, ProjectType::Resourcepack),
//                 StoreCategory::Shaderpacks => get_projects_versions(requester, slug, None, None, ProjectType::Shader)  
//             }.await?;

//         },
//         StoreType::Curseforge => {

//         }
//     }
// }

// TODO: Finish the Curseforge store and make this work properly in frontend
#[tauri::command]
pub async fn download_store_content(
    slug: &str,
    version: &str,
    instance_name: &str,
    store_type: StoreType,
    store_category: StoreCategory,
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<(), String> {
    let env = launcher_env.read().await;
    let requester = env.requester();
    let instance_path = env.instances().instance_dir(instance_name);
    let content_caching_manager = ContentCachingManager::new(&instance_path);

    match store_type {
        StoreType::Modrinth => match store_category {
            StoreCategory::Modpacks => todo!(),
            StoreCategory::Mods => download_modrinth_project(requester, &content_caching_manager, slug, version, ProjectType::Mod).await,
            StoreCategory::Shaderpacks => download_modrinth_project(requester, &content_caching_manager, slug, version, ProjectType::Shader).await,
            StoreCategory::Resourcepacks => download_modrinth_project(requester, &content_caching_manager, slug, version, ProjectType::Resourcepack).await
        }.map_err(|e| e.to_string())?,
        StoreType::Curseforge => match store_category {
            _ => todo!()
        }
    };

    Ok(())
}
