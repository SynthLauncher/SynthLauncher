use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::instances::instance_metadata::ModLoader;
use sl_utils::{errors::BackendError, fs::copy_dir_all, requester::Requester};
use zip::ZipArchive;

const MODRINTH_INDEX_NAME: &str = "modrinth.index.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DependencyLoader {
    Forge(String),
    Neoforge(String),
    FabricLoader(String),
    QuiltLoader(String),
}

impl DependencyLoader {
    pub const fn get_loader_info(&self) -> (ModLoader, &String) {
        match self {
            Self::Forge(v) => (ModLoader::Forge, v),
            Self::FabricLoader(v) => (ModLoader::Fabric, v),
            Self::Neoforge(v) => (ModLoader::NeoForge, v),
            Self::QuiltLoader(v) => (ModLoader::Quilt, v),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Dependencies {
    pub minecraft: String,
    #[serde(flatten)]
    pub loader: DependencyLoader,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthIndex {
    pub name: String,
    pub version_id: String,
    pub files: Vec<ModrinthIndexFile>,
    pub dependencies: Dependencies,
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
    requester: &Requester,
    path: &Path,
    modpack_file: &ModrinthIndexFile,
) -> Result<(), BackendError> {
    let path = path.join(&modpack_file.path);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    requester
        .builder()
        .download_to(&modpack_file.downloads[0], &path)
        .await?;

    Ok(())
}

pub(in crate::modrinth) async fn download_modpack_files(
    requester: &Requester,
    instance_path: &Path,
    modpack_files: Vec<ModrinthIndexFile>,
) -> Result<(), BackendError> {
    let tasks = FuturesUnordered::new();

    for modpack_file in &modpack_files {
        tasks.push(download_modpack_file(
            requester,
            &instance_path,
            &modpack_file,
        ));
    }

    let futures = tasks.collect::<Vec<_>>().await;
    for result in futures {
        result?;
    }

    Ok(())
}
