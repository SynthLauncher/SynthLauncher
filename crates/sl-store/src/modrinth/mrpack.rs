use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    str::FromStr,
};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::{
    launcher::instances::metadata::{InstanceMetadata, ModLoader},
    HTTP_CLIENT, INSTANCES_DIR,
};
use sl_utils::{downloader::downloader, errors::BackendError};
use zip::ZipArchive;

use crate::modrinth::api::project::query_project_version;

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

    downloader()
        .client(&HTTP_CLIENT)
        .target(&path)
        .url(&modpack_file.downloads[0])
        .call()
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

pub async fn install_modpack(slug: &str, version: &str) -> Result<(), BackendError> {
    let project_version = query_project_version(&HTTP_CLIENT, slug, version).await?;
    let instance_dir = INSTANCES_DIR.join(slug);
    let mrpack_path = instance_dir.join(&project_version.files[0].filename);
    
    tokio::fs::create_dir_all(&instance_dir).await?;
    downloader()
        .client(&HTTP_CLIENT)
        .target(&mrpack_path)
        .url(&project_version.files[0].url)
        .call()
        .await?;

    unzip_modpack(&mrpack_path, &instance_dir).await?;
    let index = read_modrinth_index(&instance_dir).await?;
    let mod_loader = ModLoader::from_str(project_version.loaders[0].as_str())?;
    let mod_loader_id = DependencyID::from(project_version.loaders[0].as_str());
    let mod_loader_version = index.dependencies.get(&mod_loader_id).cloned();
    let mc_version = index.dependencies.get(&DependencyID::Minecraft).unwrap_or(&project_version.game_versions[0]);
    
    let instance = InstanceMetadata::create(slug, &mc_version, mod_loader, mod_loader_version, None).await?;
    
    download_modpack_files(&instance_dir, &index.files).await?;

    let loaded_instance = instance.load_init().await?;
    loaded_instance.execute().await?;

    Ok(())
}
