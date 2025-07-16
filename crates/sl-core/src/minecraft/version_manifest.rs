use std::{fs, path::Path};

use sl_meta::minecraft::version_manifest::VersionManifest;
use sl_utils::{
    errors::{BackendError, InstanceError},
};

use crate::{REQUESTER, VERSION_MANIFEST, VERSION_MANIFEST_PATH};

const VERSION_MANIFEST_DOWNLOAD_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json";

pub(crate) async fn fetch_version_manifest() {
    let res = REQUESTER
        .builder()
        .download(VERSION_MANIFEST_DOWNLOAD_URL)
        .await;

    if let Ok(res) = res {
        tokio::fs::write(&VERSION_MANIFEST_PATH.as_path(), res)
            .await
            .expect("Failed writing into the file: version_manifest.json");
    }
}

pub(crate) fn read_version_manifest() -> VersionManifest {
    let buffer = fs::read_to_string(VERSION_MANIFEST_PATH.as_path())
        .expect("Failed reading the file: version_manifest.json");
    serde_json::from_str(buffer.as_str()).expect("Failed to parse file: version_manifest.json")
}

/// Downloads the client.json of a given minecraft version
pub(crate) async fn download_version_json(version: &str, to_path: &Path) -> Result<(), BackendError> {
    let Some(version) = VERSION_MANIFEST.versions().find(|x| x.id == version) else {
        // TODO: Use a different type for version instead of String
        return Err(BackendError::InstanceError(
            InstanceError::MinecraftVersionNotFound(version.to_string()),
        ));
    };

    REQUESTER
        .builder()
        .download_to(&version.url, to_path)
        .await?;

    Ok(())
}
