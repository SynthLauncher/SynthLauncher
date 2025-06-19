use std::fs;

use bytes::Bytes;
use sl_meta::minecraft::version_manifest::VersionManifest;
use sl_utils::utils::{
    self,
    errors::{BackendError, InstanceError},
};

use crate::{HTTP_CLIENT, VERSION_MANIFEST, VERSION_MANIFEST_PATH};

pub async fn fetch_version_manifest() {
    let res = utils::download::download_bytes(
        "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json",
        &HTTP_CLIENT,
        3,
        std::time::Duration::from_secs(5),
    )
    .await;

    if let Ok(res) = res {
        tokio::fs::write(&VERSION_MANIFEST_PATH.as_path(), res)
            .await
            .expect("Failed writing into the file: version_manifest.json");
    }
}

pub fn read_version_manifest() -> VersionManifest {
    let buffer = fs::read_to_string(VERSION_MANIFEST_PATH.as_path())
        .expect("Failed reading the file: version_manifest.json");
    serde_json::from_str(buffer.as_str()).expect("Failed to parse file: version_manifest.json")
}

pub async fn download_version(version: &str) -> Result<Bytes, BackendError> {
    let Some(version) = VERSION_MANIFEST.versions().find(|x| x.id == version) else {
        // TODO: Use a different type for version instead of String
        return Err(BackendError::InstanceError(
            InstanceError::VersionNotFound(version.to_string()),
        ));
    };

    let res = utils::download::download_bytes(
        &version.url,
        &HTTP_CLIENT,
        3,
        std::time::Duration::from_secs(5),
    )
    .await?;
    Ok(res)
}
