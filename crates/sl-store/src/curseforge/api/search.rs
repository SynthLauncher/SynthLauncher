use serde::{Deserialize, Serialize};
use sl_core::REQUESTER;
use sl_utils::errors::BackendError;

use crate::PAGE_SIZE;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectAsset {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectAuthor {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProject {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub download_count: u64,
    pub logo: CurseforgeProjectAsset,
    pub authors: Vec<CurseforgeProjectAuthor>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgePagination {
    pub total_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeSearchResponse {
    pub data: Vec<CurseforgeProject>,
    pub pagination: CurseforgePagination,
}

pub async fn query_curseforge_search(
    query: &str,
    class_id: u32,
    offset: u32,
) -> Result<CurseforgeSearchResponse, BackendError> {
    let url = format!(
        "https://api.curseforge.com/v1/mods/search?gameId=432&classId={}&searchFilter={}&pageSize={}&index={}&sortField=2&sortOrder=desc",
        class_id,
        query,
        PAGE_SIZE,
        offset
    );
    Ok(REQUESTER.get_json(&url).await?)
}
