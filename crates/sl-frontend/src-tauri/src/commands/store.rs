use sl_store::{
    curseforge::api::search::{query_curseforge_search, CurseforgeSearchResponse},
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
    let params = Params::new(Some(query), Some(filter), None, Some(16), Some(16 * (page - 1)));
    let search_result = query_search(params).await.map_err(|e| e.to_string())?;

    Ok(search_result)
}

#[tauri::command]
pub async fn search_curseforge_store(
    query: &str,
    offset: u32,
    class_id: u32,
) -> Result<CurseforgeSearchResponse, String> {
    let search_result = query_curseforge_search(query, class_id, (offset - 1) * 16)
        .await
        .map_err(|x| x.to_string())?;
    Ok(search_result)
}
