use sl_core::environment::LauncherEnv;
use sl_store::{
    get_content_versions, get_store_search, StoreCategory, StoreProjectVersions, StoreSearchResult,
    StoreType,
};
use tauri::State;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn fetch_store_search(
    search_query: &str,
    store_type: StoreType,
    store_category: StoreCategory,
    store_page: u32,
    launcher_env_state: State<'_, RwLock<LauncherEnv>>,
) -> Result<StoreSearchResult, String> {
    let launcher_env = launcher_env_state.read().await;
    let requester = launcher_env.requester();
    let result = get_store_search(
        store_type,
        store_category,
        search_query,
        store_page,
        requester,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn fetch_content_versions(
    store_type: StoreType,
    slug: &str,
    game_version: Option<&str>,
    loader: Option<&str>,
    launcher_env_state: State<'_, RwLock<LauncherEnv>>,
) -> Result<StoreProjectVersions, String> {
    let launcher_env = launcher_env_state.read().await;
    let requester = launcher_env.requester();
    
    let result = get_content_versions(store_type, slug, game_version, loader, requester)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}
