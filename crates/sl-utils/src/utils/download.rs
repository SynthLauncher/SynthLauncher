use std::{path::Path, time::Duration};

use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::Client;
use tokio::{io::AsyncWriteExt, sync::mpsc::Sender, time::sleep};

use crate::{elog, log};

use super::errors::HttpError;

pub async fn download_bytes(
    url: &str,
    client: &Client,
    max_retries: u32,
    duration: Duration,
    progress_tx: Option<Sender<f32>>,
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

            // Retries only if the Error is related to the response, otherwise the error is from our side
            Err(e)
                if e.is_body()
                    || e.is_decode()
                    || e.is_redirect()
                    || e.is_timeout()
                    || e.is_connect()
                    || e.is_status() =>
            {
                attempts += 1;
                log!("Retrying: download attempt {}", attempts);

                if attempts >= max_retries {
                    elog!(
                        "error while downloading '{url}' with max retries: {max_retries} (reached), reqwest error: {e}"
                    );
                    return Err(e.into());
                }

                sleep(duration).await;
            }
            Err(e) => {
                elog!("error while downloading '{url}', reqwest error: {e}");
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
    progress_tx: Option<Sender<f32>>,
) -> Result<(), HttpError> {
    let mut attempts = 0;

    loop {
        let res = client.get(url).send().await;

        match res {
            Ok(response) if response.status().is_success() => {
                let mut file = tokio::fs::File::create(dest).await?;
                let total_size = response.content_length();

                if let Some(tx) = &progress_tx {
                    if let Err(e) = tx.send(0.0).await {
                        elog!("{}", e);
                    }
                }

                let mut stream = response.bytes_stream();
                let mut downloaded: u64 = 0;

                while let Some(item) = stream.next().await {
                    let chunk = item?;
                    file.write_all(&chunk).await?;
                    downloaded += chunk.len() as u64;

                    if let Some(tx) = &progress_tx {
                        if let Err(e) = tx
                            .send((downloaded as f32 / total_size.unwrap() as f32) * 100.0)
                            .await
                        {
                            elog!("{}", e);
                        }
                    }
                }

                if let Some(tx) = &progress_tx {
                    if let Err(e) = tx.send(100.0).await {
                        elog!("{}", e);
                    }
                }

                return Ok(());
            }
            Ok(response) => return Err(HttpError::Status(response.status())),
            // Retries only if the Error is related to the response, otherwise the error is from our side
            Err(e)
                if e.is_body()
                    || e.is_decode()
                    || e.is_redirect()
                    || e.is_timeout()
                    || e.is_connect()
                    || e.is_status() =>
            {
                attempts += 1;
                log!("Retrying: download attempt {}", attempts);

                if attempts >= max_retries {
                    elog!(
                           "error while downloading '{url}' to `{}` with max retries: {max_retries} (reached), reqwest error: {e}",
                           dest.display()
                       );
                    return Err(e.into());
                }

                sleep(duration).await;
            }
            Err(e) => {
                elog!(
                    "error while downloading '{url}' to `{}`, reqwest error: {e}",
                    dest.display(),
                );
                return Err(e.into());
            }
        }
    }
}
