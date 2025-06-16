use std::io;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuiltLoaderVersion {
    pub build: u32,
    pub version: String
}

#[derive(Debug, Deserialize)]
pub struct QuiltVersion {
    pub loader: QuiltLoaderVersion
}

pub fn get_quilt_versions<F>(game_version: &str, do_request: F) -> io::Result<Vec<QuiltVersion>>
where
    F: FnOnce(&str) -> io::Result<Vec<u8>>,
{
    let response = do_request(&format!(
        "https://meta.quiltmc.org/v3/versions/loader/{}",
        game_version
    ))?;
    Ok(serde_json::from_slice(&response)?)
}
