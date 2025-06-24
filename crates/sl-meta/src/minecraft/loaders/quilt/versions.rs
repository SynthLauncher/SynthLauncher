use std::io;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuiltLoaderVersion {
    pub build: u32,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct QuiltVersion {
    pub loader: QuiltLoaderVersion,
}

/// FIXME: a minecraft version may not have any quilt versions
pub async fn get_quilt_versions<E>(
    game_version: &str,
    do_request: impl AsyncFnOnce(&str) -> Result<Vec<u8>, E>,
) -> Result<Vec<QuiltVersion>, E>
where
    E: From<io::Error>,
{
    let response = do_request(&format!(
        "https://meta.quiltmc.org/v3/versions/loader/{}",
        game_version
    ))
    .await?;
    Ok(serde_json::from_slice(&response).map_err(|e| Into::<io::Error>::into(e))?)
}
