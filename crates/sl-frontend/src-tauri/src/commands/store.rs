use sl_mod_manager::modrinth::search_query_default;
use synrinth::models::search::{QueryParams, Search};

#[tauri::command]
pub async fn search_store() -> Result<Search, String> {
    let search = search_query_default(QueryParams { query: None, facets: None }).await.map_err(|e| e.to_string())?;
    Ok(search)
}