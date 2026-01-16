use sl_core::environment::LauncherEnv;
use sl_store::{search_store, StoreContentCategory, StoreContentSearchHits, StoreSource};
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn cmd_search_store(
    query: &str,
    store_src: StoreSource,
    store_category: StoreContentCategory,
    store_page: u32,
    launcher_env_state: State<'_, RwLock<LauncherEnv>>,
) -> Result<StoreContentSearchHits, String> {
    let launcher_env = launcher_env_state.read().await;
    let requester = launcher_env.requester();
    let result = search_store(
        &requester, 
        query, 
        store_category, 
        store_page, 
        store_src
    ).await
    .map_err(|e| e.to_string())?;
    Ok(result)
}

// #[tauri::command]
// pub async fn fetch_content_versions(
//     store_type: StoreType,
//     slug: &str,
//     game_version: Option<&str>,
//     loader: Option<&str>,
//     launcher_env_state: State<'_, RwLock<LauncherEnv>>,
// ) -> Result<StoreProjectVersions, String> {
//     let launcher_env = launcher_env_state.read().await;
//     let requester = launcher_env.requester();

//     let result = get_content_versions(store_type, slug, game_version, loader, requester)
//         .await
//         .map_err(|e| e.to_string())?;

//     Ok(result)
// }

// #[tauri::command]
// pub async fn install_content(
//     instance_name: &str,
//     files: Vec<ContentFile>,
//     launcher_env_state: State<'_, RwLock<LauncherEnv>>,
// ) -> Result<(), String> {
//     let launcher_env = launcher_env_state.read().await;

//     download_content(
//         &launcher_env.requester(),
//         &launcher_env.instances(),
//         instance_name,
//         files,
//     )
//     .await
//     .map_err(|e| e.to_string())?;

//     Ok(())
// }

// #[tauri::command]
// pub async fn install_modpack(
//     slug: &str,
//     version: &str,
//     launcher_env_state: State<'_, RwLock<LauncherEnv>>,
// ) -> Result<(), String> {
//     let env = launcher_env_state.write().await;
//     download_modrinth_modpack(&mut env.instances(), slug, version)
//         .await
//         .map_err(|e| e.to_string())?;

//     Ok(())
// }
