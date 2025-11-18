use std::{collections::HashMap, path::{Path, PathBuf}};

use futures_util::{StreamExt, stream::FuturesUnordered};
use lzma_rs::lzma_decompress;
use serde::Deserialize;
use sl_meta::minecraft::loaders::vanilla::JavaComponent;
use sl_utils::{errors::BackendError, requester::Requester};

use crate::jre_manifest::JreManifest;

#[derive(Debug, Deserialize, Clone)]
pub struct JavaFile {
    executable: Option<bool>,
    r#type: String,
    // target: Option<String>,
    downloads: Option<JavaFileDownloads>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavaFileDownload {
    // sha1: String,
    url: String,
    // size: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JavaFileDownloads {
    lzma: Option<JavaFileDownload>,
    raw: Option<JavaFileDownload>,
}

#[derive(Debug, Deserialize)]
pub struct JavaFiles {
    files: HashMap<String, JavaFile>,
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

async fn download_file(
    requester: &Requester,
    path: PathBuf,
    downloads: JavaFileDownloads,
    executable: bool,
) -> Result<(), BackendError> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    if let Some(lzma) = &downloads.lzma {
        let bytes = requester.builder().download(&lzma.url).await?;

        let mut decompressed = Vec::new();
        lzma_decompress(&mut std::io::Cursor::new(&bytes), &mut decompressed)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        tokio::fs::write(&path, &decompressed).await?;
    } else if let Some(raw) = &downloads.raw {
        requester.builder().download_to(&raw.url, &path).await?;
    }

    set_executable_unix(&path, executable)?;
    Ok(())
}

pub async fn download_jre_manifest_version(
    requester: &Requester,
    jre_manifest: &JreManifest,
    dest: &Path,
    java_component: &JavaComponent,
) -> Result<(), BackendError> {
    let downloads = jre_manifest.get_component_downloads(java_component);
    let dir = dest.join(java_component.as_ref());

    let mut tasks = FuturesUnordered::new();

    for download in downloads {
        let java_files: JavaFiles = requester.get_json(&download.manifest.url).await?;

        for (file_name, java_file) in java_files.files {
            if java_file.downloads.is_none() {
                continue;
            }

            let executable = java_file.executable.unwrap_or(false);
            let path = dir.join(&file_name);
            
            if let Some(downloads) = java_file.downloads {
                tasks.push(download_file(&requester, path, downloads, executable));
            }
        }
    }

    while let Some(res) = tasks.next().await {
        res?;
    }

    Ok(())
}
