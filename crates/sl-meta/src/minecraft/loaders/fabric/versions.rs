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
pub fn get_fabric_versions<F>(game_version: &str, do_request: F) -> io::Result<Vec<FabricVersion>>
where
    F: FnOnce(&str) -> io::Result<Vec<u8>>,
{
    let response = do_request(&format!(
        "https://meta.fabricmc.net/v2/versions/loader/{}/",
        game_version
    ))?;
    Ok(serde_json::from_slice(&response)?)
}
