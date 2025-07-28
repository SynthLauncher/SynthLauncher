use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::REQUESTER;
use sl_utils::errors::BackendError;
use zip::ZipArchive;

const MODRINTH_INDEX_NAME: &'static str = "modrinth.index.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthPack {
    pub dependencies: HashMap<DependencyID, String>,
    pub files: Vec<ModrinthIndex>,
    pub format_version: u32,
    pub game: String,
    pub name: String,
    pub version_id: String,
    pub summary: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthIndex {
    pub path: PathBuf,
    pub hashes: FileHashes,
    pub env: Option<Env>,
    pub downloads: Vec<String>,
    pub file_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileHashes {
    pub sha1: String,
    pub sha512: String,
    pub other_hashes: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Env {
    pub client: EnvTypes,
    pub server: EnvTypes,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum EnvTypes {
    Required,
    Optional,
    Unsupported,
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

impl From<&str> for DependencyID {
    fn from(value: &str) -> Self {
        match value {
            "minecraft" => Self::Minecraft,
            "fabric" | "fabric-loader" => Self::FabricLoader,
            "neoforge" => Self::Neoforge,
            "quilt" | "quilt-loader" => Self::QuiltLoader,
            "forge" => Self::Forge,
            _ => panic!("This type of dependency ID doesn't exist!"),
        }
    }
}

pub async fn unzip_modpack(mrpack: &Path, output_dir: &Path) -> Result<(), BackendError> {
    let mut archive = ZipArchive::new(BufReader::new(File::open(mrpack)?))?;

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

pub async fn read_modrinth_index(modpack_path: &Path) -> Result<ModrinthPack, BackendError> {
    let json = tokio::fs::read_to_string(modpack_path.join(MODRINTH_INDEX_NAME)).await?;
    Ok(serde_json::from_str(&json)?)
}

async fn download_modpack_file(
    path: &Path,
    modpack_file: &ModrinthIndex,
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

pub async fn download_modpack_files(
    instance_path: &Path,
    modpack_files: &[ModrinthIndex],
) -> Result<(), BackendError> {
    let modpack_files = modpack_files.to_vec();
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
