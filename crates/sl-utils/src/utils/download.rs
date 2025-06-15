use std::{path::Path, time::Duration};

use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::{io::AsyncWriteExt, time::sleep};

use super::errors::HttpError;

pub async fn download_bytes(
    url: &str,
    client: &Client,
    max_retries: u32,
    duration: Duration,
) -> Result<Bytes, HttpError> {
    let mut attempts = 0;

    loop {
        let res = client.get(url).send().await;

        match res {
            Ok(response) if response.status().is_success() => {
                let bytes = response.bytes().await?;
                return Ok(bytes);
            }
            Ok(response) => return Err(HttpError::Status(response.status())),
            // retry only if Error is related to the response otherwise the error is likely from our side
            Err(e)
                if e.is_body()
                    || e.is_decode()
                    || e.is_redirect()
                    || e.is_timeout()
                    || e.is_connect()
                    || e.is_status() =>
            {
                attempts += 1;
                if attempts >= max_retries {
                    // TODO: add a logging function
                    eprintln!(
                        "error while downloading '{url}' with max retries: {max_retries} (reached), reqwest error: {e}"
                    );
                    return Err(e.into());
                }

                sleep(duration).await;
            }
            Err(e) => {
                // TODO: add a logging function
                eprintln!("error while downloading '{url}', reqwest error: {e}");
                return Err(e.into());
            }
        }
    }
}

pub async fn download_file(
    client: &Client,
    url: &str,
    dest: &Path,
    max_retries: u32,
    duration: Duration,
) -> Result<(), super::errors::HttpError> {
    let mut attempts = 0;

    loop {
        let res = client.get(url).send().await;

        match res {
            Ok(response) if response.status().is_success() => {
                let mut file = tokio::fs::File::create(dest).await?;
                let mut stream = response.bytes_stream();

                while let Some(item) = stream.next().await {
                    let chunk = item?;
                    file.write_all(&chunk).await?;
                }

                return Ok(());
            }
            Ok(response) => return Err(HttpError::Status(response.status())),
            // retry only if Error is related to the response otherwise the error is likely from our side
            Err(e)
                if e.is_body()
                    || e.is_decode()
                    || e.is_redirect()
                    || e.is_timeout()
                    || e.is_connect()
                    || e.is_status() =>
            {
                attempts += 1;
                if attempts >= max_retries {
                    // TODO: add a logging function
                    eprintln!(
                           "error while downloading '{url}' to `{}` with max retries: {max_retries} (reached), reqwest error: {e}",
                           dest.display()
                       );
                    return Err(e.into());
                }

                sleep(duration).await;
            }
            Err(e) => {
                // TODO: add a logging function
                eprintln!(
                    "error while downloading '{url}' to `{}`, reqwest error: {e}",
                    dest.display()
                );
                return Err(e.into());
            }
        }
    }
}
