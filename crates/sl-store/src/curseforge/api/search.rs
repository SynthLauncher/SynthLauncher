use serde::{Deserialize, Serialize};
use sl_utils::{errors::BackendError, requester::Requester};

use crate::{
    curseforge::api::{CurseforgePagination, CurseforgeProject},
    PAGE_SIZE,
};

pub async fn get_curseforge_search(
    requester: &Requester,
    search_filter: &str,
    class_id: u32,
    offset: u32,
) -> Result<CurseforgeSearchResponse, BackendError> {
    let url = format!(
        "https://api.curseforge.com/v1/mods/search?gameId=432&classId={}&searchFilter={}&pageSize={}&index={}&sortField=2&sortOrder=desc",
        class_id,
        search_filter,
        PAGE_SIZE,
        offset * PAGE_SIZE
    );

    let json = requester.get_json(&url).await?;

    Ok(json)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeSearchResponse {
    pub data: Vec<CurseforgeProject>,
    pub pagination: CurseforgePagination,
}
