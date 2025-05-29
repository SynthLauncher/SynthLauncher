use std::path::Path;

use bytes::Bytes;
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt; 

use super::errors::{BackendError, DownloadError};

pub async fn get_as_bytes(url: &str, client: &Client) -> Result<Bytes, DownloadError> {
    let res = client.get(url).send().await?;
    if !res.status().is_success() {
        return Err(DownloadError::Status(res.status()));
    }

    let bytes = res.bytes().await?;
    Ok(bytes)
}

pub async fn download_file(client: &Client, url: &str, path: &Path) -> Result<(), BackendError> {
    let response = client.get(url).send().await?;

    let mut file = tokio::fs::File::create(path).await?;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}
