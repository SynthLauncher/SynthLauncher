use sl_store::modrinth::search_query_default;
use sl_utils::dlog;
use synrinth::models::{search::{FacetFilter, FacetOp, FacetType, QueryParams, Search}};

#[tauri::command]
pub async fn search_store(query: &str, category: &str) -> Result<Search, String> {
    dlog!("{}", category);
    let facets = FacetFilter {
        facet: FacetType::ProjectType,
        op: FacetOp::Eq,
        value: category.to_string()
    };

    let vec = vec![vec![facets]];

    let search = search_query_default(QueryParams { query: Some(String::from(query)), facets: Some(vec) }).await.map_err(|e| e.to_string())?;
    Ok(search)
}
