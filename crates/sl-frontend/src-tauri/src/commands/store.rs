use sl_core::HTTP_CLIENT;
use sl_store::{
    facet_filters,
    modrinth::api::search::{query_search, Params, SearchResult},
};

#[tauri::command]
pub async fn search_modrinth_store(
    query: String,
    project_type: &str,
    page: u32,
) -> Result<SearchResult, String> {
    let filter = facet_filters!([ProjectType == project_type]);
    let params = Params::new(Some(query), Some(filter), None, Some(16), Some(16 * page));
    let search_result = query_search(&HTTP_CLIENT, params)
        .await
        .map_err(|e| e.to_string())?;

    Ok(search_result)
}
