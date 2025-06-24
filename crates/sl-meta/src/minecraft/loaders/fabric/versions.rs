//! wrapper for fabric's meta /v2/versions/loader/:game_version endpoint
use serde::Deserialize;
use std::io;

#[derive(Debug, Clone, Deserialize)]
pub struct FabricLoaderVersion {
    pub build: u32,
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FabricVersion {
    pub loader: FabricLoaderVersion,
}

// avoid adding deps on reqwest here
/// Fetches the Fabric versions for a given game version using the provided request function.
/// the function must return a Vec<u8> representing the response body, and must take a string parameter representing the URL.
pub async fn get_fabric_versions<F, E>(
    game_version: &str,
    do_request: F,
) -> Result<Vec<FabricVersion>, E>
where
    F: AsyncFnOnce(&str) -> Result<Vec<u8>, E>,
    E: From<io::Error>,
{
    let response = do_request(&format!(
        "https://meta.fabricmc.net/v2/versions/loader/{}/",
        game_version
    ))
    .await?;
    Ok(serde_json::from_slice(&response).map_err(|e| Into::<io::Error>::into(e))?)
}
