use std::{fs, path::{Path, PathBuf}};

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

pub async fn download_jre_manifest_version(download: JreManifestDownloadType) -> Result<(), BackendError> {
    let client = &HTTP_CLIENT;
    let downloads = JRE_MANIFEST.get_jre_manifest_download(&download);
    let download_str = download.to_string();
    let dir = JAVAS_DIR.join(download_str);

    for download in downloads {
        let res = client.get(&download.manifest.url).send().await?;
        let bytes = res.bytes().await?;
        let manifest: JavaFiles = serde_json::from_slice(&bytes)?;
        let files = manifest.java_file_by_type("file");

        for file in files {
            let original_path = dir.join(file.0);
            let java_file = file.1;

            let path = Path::new(&original_path);
            let mut components: Vec<String> = path
                .components()
                .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
                .collect();

            if let Some(filename) = components.pop() {
                let transformed_dirs: Vec<String> = components
                    .iter()
                    .map(|comp| comp.replace('.', "/"))
                    .collect();

                let mut transformed_path = PathBuf::new();
                for dir in transformed_dirs {
                    transformed_path.push(dir);
                }
                transformed_path.push(filename);

                if let Some(parent) = transformed_path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                if let Some(downloads) = java_file.downloads.as_ref() {
                    if let Some(lzma_file) = &downloads.lzma {
                        download_file(&client, &lzma_file.url, &transformed_path).await?;
                    }

                    if let Some(raw_file) = &downloads.raw {
                        download_file(&client, &raw_file.url, &transformed_path).await?;
                    }
                }
            }
        }
    }

    Ok(())
}
