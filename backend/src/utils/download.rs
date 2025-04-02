use bytes::Bytes;

#[derive(Debug)]
pub enum DownloadErr {
    Other(reqwest::Error),
    Status(reqwest::StatusCode),
    IO(std::io::Error),
    InvalidURL,
    Timeout,
}

impl From<reqwest::Error> for DownloadErr {
    fn from(err: reqwest::Error) -> Self {
        if err.is_builder() {
            DownloadErr::InvalidURL
        } else if err.is_timeout() {
            DownloadErr::Timeout
        } else if let Some(status) = err.status() {
            DownloadErr::Status(status)
        } else {
            DownloadErr::Other(err)
        }
    }
}

impl From<std::io::Error> for DownloadErr {
    fn from(value: std::io::Error) -> Self {
        DownloadErr::IO(value)
    }
}

const MAX_RETRIES: u32 = 3;
const INITIAL_BACKOFF_MS: u64 = 1000;

pub async fn get(url: &str) -> Result<Bytes, DownloadErr> {
    let mut retries = 0;
    let mut backoff_ms = INITIAL_BACKOFF_MS;

    loop {
        match reqwest::get(url).await {
            Ok(res) => {
                if !res.status().is_success() {
                    return Err(DownloadErr::Status(res.status()));
                }
                match res.bytes().await {
                    Ok(bytes) => return Ok(bytes),
                    Err(e) => {
                        if retries >= MAX_RETRIES {
                            return Err(DownloadErr::Other(e));
                        }
                    }
                }
            }
            Err(e) => {
                if retries >= MAX_RETRIES {
                    return Err(e.into());
                }
            }
        }

        retries += 1;
        tokio::time::sleep(std::time::Duration::from_millis(backoff_ms)).await;
        backoff_ms *= 2; // Exponential backoff
    }
}
