use sl_java_manager::jre_manifest::JreManifest;
use std::fs::{self};

use crate::{JRE_MANIFEST_PATH, REQUESTER};

pub const JRE_MANIFEST_DOWNLOAD_URL: &'static str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";

pub async fn fetch_jre_manifest() {
    let res = REQUESTER
        .builder()
        .download(JRE_MANIFEST_DOWNLOAD_URL)
        .await;

    if let Ok(res) = res {
        tokio::fs::write(&JRE_MANIFEST_PATH.as_path(), res)
            .await
            .expect("Failed writing into the file: jre_manifest.json");
    }
}

pub fn read_jre_manifest() -> JreManifest {
    let buffer = fs::read_to_string(&JRE_MANIFEST_PATH.as_path())
        .expect("Failed reading the file: jre_manifest.json");
    serde_json::from_str(buffer.as_str()).expect("Failed to parse file: jre_manifest.json")
}
