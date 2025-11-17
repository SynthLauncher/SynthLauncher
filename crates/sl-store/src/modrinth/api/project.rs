use sl_utils::{errors::BackendError, requester::Requester};
use std::fmt::Write;

use crate::modrinth::api::ModrinthProject;

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

pub async fn get_modrinth_project_versions(
    requester: &Requester,
    slug: &str,
    game_version: Option<&str>,
    loader: Option<&str>,
) -> Result<Vec<ModrinthProject>, BackendError> {
    let mut url = format!("https://api.modrinth.com/v2/project/{}/version?", slug);

    if let Some(game_version) = game_version {
        _ = write!(url, "game_versions=[\"{}\"]&", game_version);
    }

    if let Some(loader) = loader {
        _ = write!(url, "loaders=[\"{}\"]", loader);
    }

    Ok(requester.get_json(&url).await?)
}
