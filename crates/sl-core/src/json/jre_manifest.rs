use std::fs;

use sl_meta::json::jre_manifest::{JavaFiles, JreManifest, JreManifestDownloadType};
use sl_utils::utils::{self, download::download_file, errors::BackendError};

use crate::{HTTP_CLIENT, JAVAS_DIR, JRE_MANIFEST, JRE_MANIFEST_PATH};

pub async fn fetch_jre_manifest() {
    let res = utils::download::get_as_bytes(
        "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json",
        &HTTP_CLIENT,
    )
    .await;

    if let Ok(res) = res {
        tokio::fs::write(&JRE_MANIFEST_PATH.as_path(), res)
            .await
            .expect("Failed writing into the file: jre_manifest.json");
    }
}

pub fn jre_manifest_read() -> JreManifest {
    let buffer = fs::read_to_string(&JRE_MANIFEST_PATH.as_path())
        .expect("Failed reading the file: jre_manifest.json");
    serde_json::from_str(buffer.as_str()).expect("Failed to parse file: jre_manifest.json")
}

pub async fn download_jre_manifest_version(
    download: JreManifestDownloadType,
) -> Result<(), BackendError> {
    let client = &HTTP_CLIENT;
    let downloads = JRE_MANIFEST.get_jre_manifest_download(&download);
    let download_str = download.to_string();
    let dir = JAVAS_DIR.join(download_str);

    for download in downloads {
        let res = client.get(&download.manifest.url).send().await?;
        let bytes = res.bytes().await?;
        let manifest: JavaFiles = serde_json::from_slice(&bytes)?;
        let files = manifest.java_file_by_type("file");

        for (file, java_file) in files {
            let path = dir.join(file);

            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            if let Some(downloads) = java_file.downloads.as_ref() {
                if let Some(raw_file) = &downloads.raw {
                    download_file(&client, &raw_file.url, &path).await?;
                }
            }
        }
    }

    Ok(())
}
