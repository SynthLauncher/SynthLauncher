use sl_utils::{errors::BackendError, requester::Requester};
use std::fmt::Write;

use crate::modrinth::api::ModrinthProject;

#[must_use]
pub async fn get_modrinth_project_versions(
    requester: &Requester,
    slug: &str,
    game_version: Option<&str>,
    loader: Option<&str>,
) -> Result<Vec<ModrinthProject>, BackendError> {
    let mut url = format!("https://api.modrinth.com/v2/project/{}/version?", slug);
    let mut query_params = Vec::new();

    if let Some(game_version) = game_version {
        query_params.push(format!("game_versions=[\"{}\"]", game_version));
    }

    if let Some(loader) = loader {
        query_params.push(format!("loaders=[\"{}\"]", loader.to_lowercase()));
    }

    _ = write!(url, "{}", query_params.join("&"));

    let json = requester.get_json(&url).await?;
    Ok(json)
}

#[must_use]
pub async fn get_modrinth_project(
    requester: &Requester,
    slug: &str,
    version: &str,
) -> Result<ModrinthProject, BackendError> {
    let url = format!(
        "https://api.modrinth.com/v2/project/{}/version/{}",
        slug, version
    );
    Ok(requester.get_json(&url).await?)
}
