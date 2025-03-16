use bytes::Bytes;

#[derive(Debug)]
pub enum DownloadErr {
    Other(reqwest::Error),
    Status(reqwest::StatusCode),
    IO(std::io::Error),
    InvalidURL,
    Timeout
}

impl From<reqwest::Error> for DownloadErr {
    fn from(err: reqwest::Error) -> Self {
        if err.is_builder() {
            DownloadErr::InvalidURL
        }
        else if err.is_timeout() {
            DownloadErr::Timeout
        }
        else if let Some(status) = err.status() {
            DownloadErr::Status(status)
        }
        else {
            DownloadErr::Other(err)
        }
    }
}

impl From<std::io::Error> for DownloadErr {
    fn from(value: std::io::Error) -> Self {
        DownloadErr::IO(value)
    }
}

pub async fn get(url: &str) -> Result<Bytes, DownloadErr> {
    let res = reqwest::get(url).await?;
    if !res.status().is_success() {
        return Err(DownloadErr::Status(res.status()));
    }
    let bytes = res.bytes().await?;
    Ok(bytes)
}