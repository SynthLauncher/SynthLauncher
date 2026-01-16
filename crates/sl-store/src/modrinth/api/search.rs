use serde::{Deserialize, Serialize};
use sl_utils::{errors::HttpError, requester::Requester};

use crate::{modrinth::api::ProjectType, PAGE_SIZE};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthSearchHit {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub project_id: String,
    pub project_type: ProjectType,
    pub downloads: u32,
    pub icon_url: Option<String>,
    pub author: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthSearchHits {
    pub hits: Vec<ModrinthSearchHit>,
    pub total_hits: u32,
}

pub async fn search_modrinth_store(
    requester: &Requester,
    query: &str,
    project_type: &str,
    offset: u32,
) -> Result<ModrinthSearchHits, HttpError> {
    let url = format!(
        "https://api.modrinth.com/v2/search?facets=[[\"project_type:{}\"]]&limit={}&offset={}&query={}",
        project_type,
        PAGE_SIZE,
        offset * PAGE_SIZE,
        query
    );

    Ok(requester.get_json(&url).await?)
}
