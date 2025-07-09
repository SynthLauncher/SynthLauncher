use sl_java_manager::jre_manifest::JreManifest;
use sl_utils::downloader::downloader;
use std::fs::{self};

use crate::{HTTP_CLIENT, JRE_MANIFEST_PATH};

pub async fn fetch_jre_manifest() {
    let res = downloader()
        .client(&HTTP_CLIENT)
        .url("https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json")
        .call()
        .await;

    if let Ok(Some(res)) = res {
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
