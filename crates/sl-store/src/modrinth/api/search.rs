use sl_utils::{errors::BackendError, requester::Requester};

use crate::{modrinth::api::ModrinthSearchResult, PAGE_SIZE};

pub async fn get_modrinth_search(
    requester: &Requester,
    query: &str,
    project_type: &str,
    offset: u32,
) -> Result<ModrinthSearchResult, BackendError> {
    let url = format!(
        "https://api.modrinth.com/v2/search?facets=[[\"project_type:{}\"]]&limit={}&offset={}&query={}",
        project_type,
        PAGE_SIZE,
        offset * PAGE_SIZE,
        query
    );

    let json = requester.get_json(&url).await?;
    Ok(json)
}
