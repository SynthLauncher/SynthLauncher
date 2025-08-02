use std::{io, path::Path};

use sl_meta::minecraft::version_manifest::VersionManifest;
use sl_utils::{
    errors::{BackendError, HttpError, InstanceError},
    requester::Requester,
};

const VERSION_MANIFEST_DOWNLOAD_URL: &str =
    "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

/// Attempts to fetch a new version manifest or read it from a given path if fetch fails
pub async fn try_fetch_or_read(requester: &Requester, path: &Path) -> io::Result<VersionManifest> {
    match fetch_version_manifest_to(requester, path).await {
        Err(_) => read(path).await,
        Ok(results) => Ok(results),
    }
}

async fn fetch_version_manifest_to(
    requester: &Requester,
    path: &Path,
) -> Result<VersionManifest, HttpError> {
    requester
        .builder()
        .download_to(VERSION_MANIFEST_DOWNLOAD_URL, path)
        .await?;
    let manifest = read(path).await?;
    Ok(manifest)
}

async fn read(path: &Path) -> io::Result<VersionManifest> {
    let data = tokio::fs::read(path).await?;
    let parsed = serde_json::from_slice::<VersionManifest>(&data)?;
    Ok(parsed)
}

/// Downloads the client.json of a given minecraft version
pub(crate) async fn download_version_json(
    manifest: &VersionManifest,
    requester: &Requester,
    version: &str,
    to_path: &Path,
) -> Result<(), BackendError> {
    let Some(version) = manifest.get_version_by_id(version) else {
        // TODO: Use a different type for version instead of String
        return Err(BackendError::InstanceError(
            InstanceError::MinecraftVersionNotFound(version.to_string()),
        ));
    };

    requester
        .builder()
        .download_to(&version.url, to_path)
        .await?;

    Ok(())
}
