use std::{fs::File, io::Write, path::Path};

use bytes::Bytes;

use super::errors::{BackendError, DownloadError};

/*
    For SynthLauncher Core
*/
pub async fn get_as_bytes(url: &str) -> Result<Bytes, DownloadError> {
    let res = reqwest::get(url).await?;
    if !res.status().is_success() {
        return Err(DownloadError::Status(res.status()));
    }

    let bytes = res.bytes().await?;
    Ok(bytes)
}

/*
    For Java Manager
*/
pub async fn download_file(url: &str, path: &Path) -> Result<(), BackendError> {
    let mut res = reqwest::get(url).await?;
    let mut file = File::create(path)?;

    while let Some(chunk) = res.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}
