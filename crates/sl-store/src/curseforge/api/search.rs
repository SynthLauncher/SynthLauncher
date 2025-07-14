use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use sl_core::HTTP_CLIENT;
use sl_utils::errors::BackendError;

use crate::curseforge::api::{project::Project, MINECRAFT_GAME_ID, PAGE_SIZE};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub page_index: u32,
    pub class_id: u32,
    pub search_filter: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub data: Vec<Project>,
}

pub async fn query_search(req: SearchRequest) -> Result<SearchResponse, BackendError> {
    let url = format!(
        "https://api.curseforge.com/v1/mods/search?gameId={}&classId={}&searchFilter={}&pageSize={}&index={}",
        MINECRAFT_GAME_ID,
        req.class_id,
        req.search_filter,
        PAGE_SIZE,
        req.page_index,
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-api-key",
        HeaderValue::from_static("$2a$10$/Dc9lilNTw0EvobjzoQLWu7zJpqX38hahG/jugi41F39z08R1rMZC"),
    );

    let res = HTTP_CLIENT
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
