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
    duration: Duration
) -> Result<Bytes, HttpError> {
    let mut attemps = 0;

    while attemps < max_retries {
        let res = client.get(url).send().await;

        match res {
            Ok(response) if response.status().is_success() => {
                let bytes = response.bytes().await?;
                return Ok(bytes);
            },
            Ok(response) => {
                return Err(HttpError::Status(response.status()));
            },
            Err(_) => {
                attemps += 1;
                if attemps >= max_retries {
                    return Err(HttpError::MaxRetriesExceeded);
                }

                sleep(duration).await;
            }
        }
    }

    Err(HttpError::RetryFailed)
}

pub async fn download_file(
    client: &Client,
    url: &str,
    dest: &Path,
    max_retries: u32,
    duration: Duration,
) -> Result<(), super::errors::HttpError> {
    let mut attemps = 0;

    while attemps < max_retries {
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
            },
            Ok(response) => {
                return Err(HttpError::Status(response.status()));
            },
            Err(_) => {
                attemps += 1;
                if attemps >= max_retries {
                    return Err(HttpError::MaxRetriesExceeded);
                }

                sleep(duration).await;
            }
        }
    }

    Err(HttpError::RetryFailed)
}
