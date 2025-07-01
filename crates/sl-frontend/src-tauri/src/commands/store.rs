use sl_store::modrinth::search_query_default;
use synrinth::models::search::{FacetFilter, FacetOp, FacetType, QueryParams, Search};

#[tauri::command]
pub async fn search_store(query: &str, category: &str) -> Result<Search, String> {
    let facets = FacetFilter {
        facet: FacetType::ProjectType,
        op: FacetOp::Eq,
        value: category.to_string(),
    };

    let inner = vec![facets];
    let facets_ref: &[&[FacetFilter]] = &[&inner];

    let search = search_query_default(QueryParams {
        query: Some(query),
        facets: Some(&facets_ref),
        index: None,
        limit: Some(10),
        offset: None,
    })
    .await
    .map_err(|e| e.to_string())?;
    Ok(search)
}
