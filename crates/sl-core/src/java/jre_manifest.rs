use std::{fs::{self, File}, io::{Cursor, Write}, os::unix::fs::PermissionsExt};

use lzma_rs::lzma_decompress;
use sl_meta::java::jre_manifest::{JavaFiles, JreManifest, JreManifestDownloadType};
use sl_utils::{utils::{self, download::download_file, errors::BackendError}};

use crate::{HTTP_CLIENT, JAVAS_DIR, JRE_MANIFEST, JRE_MANIFEST_PATH};

pub async fn fetch_jre_manifest() {
    let res = utils::download::download_bytes(
        "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json",
        &HTTP_CLIENT,
        3,
        std::time::Duration::from_secs(5),
    )
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

pub async fn download_jre_manifest_version(
    download_type: &JreManifestDownloadType,
) -> Result<(), BackendError> {
    let downloads = JRE_MANIFEST.get_jre_manifest_download(download_type);
    let dir = JAVAS_DIR.join(download_type.to_string());

    for download in downloads {
        let res = HTTP_CLIENT.get(&download.manifest.url).send().await?;
        let java_files: JavaFiles = res.json().await?;

        for (file_name, java_file) in java_files.files {
            let path = dir.join(file_name);

            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            if let Some(downloads) = java_file.downloads.as_ref() {
                if let Some(lzma_file) = &downloads.lzma {
                    let compressed = HTTP_CLIENT.get(&lzma_file.url).send().await?.bytes().await?;

                    let mut decompressed = Vec::new();
                    lzma_decompress(&mut Cursor::new(&compressed), &mut decompressed)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                    let mut out_file = File::create(&path)?;
                    out_file.write_all(&decompressed)?;

                    if java_file.executable == Some(true) {
                        fs::set_permissions(&path, fs::Permissions::from_mode(0o777))?;
                    }
                } else if let Some(raw_file) = &downloads.raw {
                    download_file(&HTTP_CLIENT, &raw_file.url, &path, 3, std::time::Duration::from_secs(5)).await?;
                    if java_file.executable == Some(true) {
                        fs::set_permissions(&path, fs::Permissions::from_mode(0o777))?;
                    }
                }
            }
        }
    }

    Ok(())
}