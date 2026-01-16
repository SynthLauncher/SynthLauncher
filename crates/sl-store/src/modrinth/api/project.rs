use sl_utils::{errors::{BackendError, HttpError}, requester::Requester};
use std::fmt::Write;

use crate::modrinth::api::{ModrinthContent, ModrinthProject};

pub async fn get_modrinth_project(
    requester: &Requester,
    slug: &str
) -> Result<ModrinthContent, BackendError> {
    let url = format!(
        "https://api.modrinth.com/v2/project/{}",
        slug
    );
    Ok(requester.get_json(&url).await?)
}

pub async fn get_modrinth_project_version(
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
        _ = write!(url, "loaders=[\"{}\"]", loader.to_lowercase());
    }

    println!("{}", url);
    Ok(requester.get_json(&url).await?)
}

pub async fn get_modrinth_project_from_hash(
    requester: &Requester,
    hash: &str,
) -> Result<ModrinthProject, BackendError>
{
    let url = format!("https://api.modrinth.com/v2/version_file/{}", hash);
    let res: Result<ModrinthProject, HttpError> = requester.get_json(&url).await;
    match &res {
        Ok(_) => {},
        Err(err) => println!("Error at {}: {:?}", url, err)
    }

    Ok(res?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_modrinth_project_from_hash_1() -> Result<(), BackendError> {
        let requester = Requester::new();
        let hash = "922d2c76a063f18c6d24fee156da2b883afdf4be";
        let project = get_modrinth_project_from_hash(&requester, &hash).await?;
        assert_eq!(&project.name, "BetterGrassify 1.8.2+fabric.1.21.10");
        assert_eq!(&project.id, "2C7y66BK");
        Ok(())
    }
}
