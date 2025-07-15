use std::{collections::HashMap, path::Path, sync::Arc};

use futures_util::{StreamExt, stream::FuturesUnordered};
use lzma_rs::lzma_decompress;
use serde::Deserialize;
use sl_meta::minecraft::loaders::vanilla::JavaComponent;
use sl_utils::{
    errors::BackendError, requester::Requester,
};

use crate::jre_manifest::JreManifest;

#[derive(Debug, Deserialize, Clone)]
pub struct JavaFile {
    pub executable: Option<bool>,
    pub r#type: String,
    pub target: Option<String>,
    pub downloads: Option<JavaFileDownloads>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavaFileDownload {
    pub sha1: String,
    pub url: String,
    pub size: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavaFileDownloads {
    pub lzma: Option<JavaFileDownload>,
    pub raw: Option<JavaFileDownload>,
}

#[derive(Debug, Deserialize)]
pub struct JavaFiles {
    pub files: HashMap<String, JavaFile>,
}

impl JavaFiles {
    pub fn java_file_excluding_type<'a>(
        &'a self,
        r#type: &'a str,
    ) -> impl Iterator<Item = (&'a String, &'a JavaFile)> + 'a {
        self.files
            .iter()
            .filter(move |(_, file)| file.r#type != r#type)
    }
}

fn set_executable_unix(path: &Path, executable: bool) -> std::io::Result<()> {
    #[cfg(unix)]
    if executable {
        use std::os::unix::fs::PermissionsExt;

        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o777))?;
    }
    Ok(())
}

pub async fn download_jre_manifest_version(
    requester: &Requester,
    jre_manifest: &JreManifest,
    dest: &Path,
    java_component: &JavaComponent,
) -> Result<(), BackendError> {
    let downloads = jre_manifest.get_component_downloads(java_component);
    let dir = Arc::new(dest.join(java_component.to_string()));
    let requester = Arc::new(requester.clone());

    let mut tasks = FuturesUnordered::new();

    for download in downloads {
        let java_files: JavaFiles = requester.get_json(&download.manifest.url).await?;
        
        for (file_name, java_file) in java_files.files {
            if java_file.downloads.is_none() {
                continue;
            }

            let requester = Arc::<Requester>::clone(&requester);
            let dir = Arc::clone(&dir);
            let downloads = java_file.downloads.clone().unwrap();
            let executable = java_file.executable.unwrap_or(false);
            let file_name = file_name.clone();

            tasks.push(tokio::spawn(async move {
                let path = dir.join(&file_name);

                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                if let Some(lzma) = downloads.lzma {
                    let bytes = requester
                        .builder()
                        .download(&lzma.url)
                        .await?;

                    let mut decompressed = Vec::new();
                    lzma_decompress(&mut std::io::Cursor::new(&bytes), &mut decompressed)
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                    std::fs::write(&path, &decompressed)?;
                } else if let Some(raw) = downloads.raw {
                    requester
                        .builder()
                        .download_to(&raw.url, &path)
                        .await?;
                }

                set_executable_unix(&path, executable)?;
                Ok::<_, BackendError>(())
            }));
        }
    }

    while let Some(res) = tasks.next().await {
        res??;
    }

    Ok(())
}
