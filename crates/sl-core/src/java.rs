use sl_java_manager::jre_manifest::JreManifest;
use sl_utils::requester::Requester;
use std::{io, path::Path};

const JRE_MANIFEST_DOWNLOAD_URL: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";

/// Fetches the JRE manifest using the given requester or if it fails, reads it from the given path.
pub async fn try_fetch_jre_manifest_or_read(
    requester: &Requester,
    path: &Path,
) -> io::Result<JreManifest> {
    match fetch_jre_manifest_to(requester, path).await {
        Err(_) => read_jre_manifest_from(path).await,
        Ok(k) => Ok(k),
    }
}

/// Fetches the JRE manifest to the given path.
async fn fetch_jre_manifest_to(
    requster: &Requester,
    to: &Path,
) -> Result<JreManifest, sl_utils::errors::HttpError> {
    requster
        .builder()
        .download_to(JRE_MANIFEST_DOWNLOAD_URL, to)
        .await?;
    let results = read_jre_manifest_from(to).await?;
    Ok(results)
}

/// Fetches the JRE manifest from the given path.
async fn read_jre_manifest_from(path: &Path) -> io::Result<JreManifest> {
    let buf = tokio::fs::read(path).await?;
    let manifest = serde_json::from_slice(&buf)?;
    Ok(manifest)
}
