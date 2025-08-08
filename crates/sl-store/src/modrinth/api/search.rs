use serde::{Deserialize, Serialize};
use sl_utils::{errors::BackendError, requester::Requester};

use crate::PAGE_SIZE;

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthSearchHit {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub project_type: String,
    pub downloads: u32,
    pub icon_url: Option<String>,
    pub project_id: String,
    pub author: String,
    pub versions: Vec<String>,
    pub follows: u32,
    pub latest_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub hits: Vec<ModrinthSearchHit>,
    pub total_hits: u32
}

pub async fn get_modrinth_search(query: &str, project_type: &str, offset: u32) -> Result<SearchResult, BackendError> {
    let url = format!(
        "https://api.modrinth.com/v2/search?facets=[[\"project_type:{}\"]]&limit={}&offset={}&query={}",
        project_type,
        PAGE_SIZE,
        offset,
        query
    );

    let json = Requester::new().get_json(&url).await?;
    Ok(json)
}
