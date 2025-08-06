use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::{launcher::instances::instance_metadata::ModLoader, REQUESTER};
use sl_utils::{errors::BackendError, fs::copy_dir_all};
use zip::ZipArchive;

const MODRINTH_INDEX_NAME: &str = "modrinth.index.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthIndex {
    pub name: String,
    pub version_id: String,
    pub files: Vec<ModrinthIndexFile>,
    pub dependencies: HashMap<DependencyID, String>,
}

impl ModrinthIndex {
    pub fn get_mod_loader(&self) -> Option<(ModLoader, String)> {
        if let Some(v) = self.dependencies.get(&DependencyID::Forge) {
            return Some((ModLoader::Forge, v.to_string()));
        }
        if let Some(v) = self.dependencies.get(&DependencyID::Neoforge) {
            return Some((ModLoader::NeoForge, v.to_string()));
        }
        if let Some(v) = self.dependencies.get(&DependencyID::FabricLoader) {
            return Some((ModLoader::Fabric, v.to_string()));
        }
        if let Some(v) = self.dependencies.get(&DependencyID::QuiltLoader) {
            return Some((ModLoader::Quilt, v.to_string()));
        }

        None
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthIndexFile {
    pub path: PathBuf,
    pub hashes: FileHashes,
    pub downloads: Vec<String>,
    // pub file_size: u32, // Maybe this will be needed for progress tracking?
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileHashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DependencyID {
    Minecraft,
    Forge,
    Neoforge,
    FabricLoader,
    QuiltLoader,
}

pub(in crate::modrinth) async fn unzip_modpack(
    modpack_zip_path: &Path,
    output_dir: &Path,
) -> Result<(), BackendError> {
    let mut archive = ZipArchive::new(BufReader::new(File::open(modpack_zip_path)?))?;

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

pub(in crate::modrinth) async fn read_modrinth_index(
    modpack_path: &Path,
) -> Result<ModrinthIndex, BackendError> {
    let bytes = tokio::fs::read(modpack_path.join(MODRINTH_INDEX_NAME)).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub(in crate::modrinth) fn copy_overrides(
    modpack_path: &Path,
    instance_path: &Path,
) -> Result<(), BackendError> {
    let overrides = modpack_path.join("overrides");
    copy_dir_all(overrides, instance_path)?;
    Ok(())
}

async fn download_modpack_file(
    path: &Path,
    modpack_file: &ModrinthIndexFile,
) -> Result<(), BackendError> {
    let path = path.join(&modpack_file.path);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    REQUESTER
        .builder()
        .download_to(&modpack_file.downloads[0], &path)
        .await?;

    Ok(())
}

pub(in crate::modrinth) async fn download_modpack_files(
    instance_path: &Path,
    modpack_files: Vec<ModrinthIndexFile>,
) -> Result<(), BackendError> {
    let tasks = FuturesUnordered::new();

    for modpack_file in &modpack_files {
        tasks.push(download_modpack_file(&instance_path, &modpack_file));
    }

    let futures = tasks.collect::<Vec<_>>().await;
    for result in futures {
        result?;
    }

    Ok(())
}
