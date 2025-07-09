use std::{path::Path, time::Duration};
use bon::builder;
use bytes::Bytes;
use reqwest::Client;
use tokio::{fs::File, io::AsyncWriteExt, sync::mpsc::Sender, time::sleep};
use tokio_stream::StreamExt;

use crate::{log, utils::errors::HttpError};

pub async fn retry<T, F, Fut>(
    mut f: F,
    max_retries: u32,
    delay: Duration,
    on_retry: impl Fn(u32),
) -> Result<T, HttpError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, HttpError>>,
{
    let mut attempts = 0;
    loop {
        match f().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                attempts += 1;
                on_retry(attempts);
                if attempts >= max_retries {
                    return Err(e);
                }
                sleep(delay).await;
            }
        }
    }
}

#[builder]
pub async fn downloader(
    client: &Client,
    url: &str,
    target: Option<&Path>,
    #[builder(default = 3)]
    max_retries: u32,
    #[builder(default = std::time::Duration::from_secs(5))]
    delay: Duration,
    progress_tx: Option<Sender<f32>>,
) -> Result<Option<Bytes>, HttpError> {
    retry(
        || {
            let client = client.clone();
            let url = url.to_owned();
            let target = target.map(|p| p.to_path_buf());
            let progress_tx = progress_tx.clone();

            async move {
                let response = client.get(url).send().await?;
                if !response.status().is_success() {
                    return Err(HttpError::Status(response.status()));
                }

                if let Some(path) = target {
                    let mut file = File::create(&path).await?;
                    let total_size = response.content_length().unwrap_or(0);
                    let mut downloaded = 0u64;
                    let mut stream = response.bytes_stream();

                    if let Some(tx) = &progress_tx {
                        let _ = tx.send(0.0).await;
                    }

                    while let Some(chunk) = stream.next().await {
                        let chunk = chunk?;
                        file.write_all(&chunk).await?;
                        downloaded += chunk.len() as u64;

                        if let Some(tx) = &progress_tx {
                            if total_size > 0 {
                                let percent = (downloaded as f32 / total_size as f32) * 100.0;
                                let _ = tx.send(percent).await;
                            }
                        }
                    }

                    if let Some(tx) = &progress_tx {
                        let _ = tx.send(100.0).await;
                    }

                    Ok(None)
                } else {
                    let bytes = response.bytes().await?;
                    if let Some(tx) = &progress_tx {
                        let _ = tx.send(100.0).await;
                    }
                    Ok(Some(bytes))
                }
            }
        },
        max_retries,
        delay,
        |attempt| log!("Retrying '{}', attempt {}", url, attempt),
    )
    .await
}
