use std::{fs::File, io::Write, path::Path};

use crate::utils::errors::BackendError;

pub async fn download_file(url: &str, path: &Path) -> Result<(), BackendError> {
    let client = reqwest::Client::new();
    let mut response = client.get(url).send().await?;

    let mut file = File::create(path)?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}
