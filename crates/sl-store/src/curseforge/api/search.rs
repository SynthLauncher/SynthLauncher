use serde::{Deserialize, Serialize};
use sl_core::REQUESTER;
use sl_utils::errors::BackendError;

use crate::{
    curseforge::api::{project::CurseforgeProject, MINECRAFT_GAME_ID},
    PAGE_SIZE,
};

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
        "https://api.curseforge.com/v1/mods/search?gameId={MINECRAFT_GAME_ID}&classId={class_id}&searchFilter={query}&pageSize={PAGE_SIZE}&index={offset}&sortField=2&sortOrder=\"desc\"",
    );

    let res: CurseforgeSearchResponse = REQUESTER.get_json(&url).await?;

    Ok(res)
}
