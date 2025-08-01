use std::{fs::File, io::BufReader, path::Path};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::REQUESTER;
use sl_utils::errors::BackendError;
use zip::ZipArchive;

use crate::curseforge::api::project::query_project_file;

const MODPACK_MANIFEST_NAME: &str = "manifest.json";

#[derive(Debug, Deserialize)]
pub struct ModLoader {
    pub id: String,
    pub primary: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Minecraft {
    pub version: String,
    pub mod_loaders: Vec<ModLoader>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModpackFile {
    #[serde(rename = "projectID")]
    pub project_id: u32,
    #[serde(rename = "fileID")]
    pub file_id: u32,
    pub required: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModpackManifest {
    pub minecraft: Minecraft,
    pub name: String,
    pub author: String,
    pub files: Vec<ModpackFile>,
}

pub async fn unzip_modpack(modpack: &Path, output_dir: &Path) -> Result<(), BackendError> {
    let mut archive = ZipArchive::new(BufReader::new(File::open(modpack)?))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = output_dir.join(file.name());

        if file.is_dir() {
            tokio::fs::create_dir_all(&out_path).await?;
        } else {
            if let Some(parent) = out_path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            std::io::copy(&mut file, &mut File::create(&out_path)?)?;
        }
    }

    Ok(())
}

pub async fn read_modpack_manifest(modpack_path: &Path) -> Result<ModpackManifest, BackendError> {
    let json = tokio::fs::read_to_string(modpack_path.join(MODPACK_MANIFEST_NAME)).await?;
    Ok(serde_json::from_str(&json)?)
}

async fn download_modpack_file(
    mods_folder: &Path,
    modpack_file: &ModpackFile,
) -> Result<(), BackendError> {
    let project_file = query_project_file(modpack_file.project_id, modpack_file.file_id).await?;
    let path = mods_folder.join(project_file.data.file_name);

    if let Some(download_url) = project_file.data.download_url {
        REQUESTER
            .builder()
            .download_to(&download_url, &path)
            .await?;
    }

    Ok(())
}

pub async fn download_modpack_files(
    instance_path: &Path,
    modpack_files: Vec<ModpackFile>,
) -> Result<(), BackendError> {
    let mut tasks = FuturesUnordered::new();

    for modpack_file in modpack_files {
        let instance_path = instance_path.to_path_buf();

        tasks.push(tokio::spawn(async move {
            download_modpack_file(&instance_path, &modpack_file).await
        }));
    }

    while let Some(result) = tasks.next().await {
        result??;
    }

    Ok(())
}
