use sl_utils::{errors::BackendError, requester::Requester};

use crate::{modrinth::api::{ModrinthSearchResult, ProjectType}, PAGE_SIZE};

pub async fn get_modrinth_search(
    requester: &Requester,
    query: &str,
    project_type: ProjectType,
    offset: u32,
) -> Result<ModrinthSearchResult, BackendError> {
    let url = format!(
        "https://api.modrinth.com/v2/search?facets=[[\"project_type:{}\"]]&limit={}&offset={}&query={}",
        <&'static str>::from(project_type),
        PAGE_SIZE,
        offset * PAGE_SIZE,
        query
    );

    Ok(requester.get_json(&url).await?)
}
