use std::{fs::File, io::Write, path::Path, time::Duration};
use tokio::time::sleep;
use crate::utils::errors::BackendError;
use reqwest::Client;
use bytes::Bytes;

async fn download_with_retry(client: &Client, url: &str, retries: u32, initial_delay: Duration) -> Result<Bytes, BackendError> {
    let mut delay = initial_delay;
    let mut attempt = 0;
    let mut last_error = None;

    while attempt < retries {
        attempt += 1;
        println!("Download attempt {} for {}", attempt, url);

        match client.get(url)
            .timeout(Duration::from_secs(30)) // Per-request timeout
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.bytes().await {
                        Ok(bytes) => return Ok(bytes),
                        Err(e) => {
                            println!("Failed to read response bytes: {}", e);
                            last_error = Some(BackendError::from(e));
                        }
                    }
                } else {
                    println!("Server returned error status: {}", response.status());
                    last_error = Some(BackendError::DownlaodError(
                        crate::utils::download::DownloadErr::Status(response.status())
                    ));
                }
            }
            Err(e) => {
                println!("Request failed: {}", e);
                last_error = Some(BackendError::from(e));
            }
        }

        if attempt < retries {
            println!("Retrying in {} seconds...", delay.as_secs());
            sleep(delay).await;
            delay = std::cmp::min(delay * 2, Duration::from_secs(60)); // Cap max delay at 60 seconds
        }
    }

    Err(last_error.unwrap_or_else(|| {
        BackendError::DownlaodError(crate::utils::download::DownloadErr::InvalidURL)
    }))
}

pub async fn download_file(url: &str, path: &Path) -> Result<(), BackendError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60)) // Increased timeout
        .pool_idle_timeout(Duration::from_secs(90)) // Keep connections alive longer
        .pool_max_idle_per_host(5) // Allow more idle connections per host
        .tcp_keepalive(Duration::from_secs(60)) // Enable TCP keepalive
        .build()
        .map_err(|e| BackendError::from(e))?;

    let retries = 5; // Increased retries
    let initial_delay = Duration::from_secs(2); // Increased initial delay

    let bytes = download_with_retry(&client, url, retries, initial_delay).await?;

    // Create a temporary file first
    let temp_path = path.with_extension("temp");
    let mut temp_file = File::create(&temp_path)?;
    temp_file.write_all(&bytes)?;
    temp_file.sync_all()?;

    // Only after successful write, rename the temp file to the target file
    std::fs::rename(temp_path, path)?;

    Ok(())
}
