use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::instances::{instance_metadata::ModLoader, InstanceManager};
use sl_utils::{errors::BackendError, fs::copy_dir_all, requester::Requester};
use tempdir::TempDir;
use zip::ZipArchive;

use crate::modrinth::api::project::get_modrinth_project;

const MODRINTH_INDEX_NAME: &str = "modrinth.index.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum DependencyLoader {
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
struct Dependencies {
    pub minecraft: String,
    #[serde(flatten)]
    pub loader: DependencyLoader,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModrinthIndex {
    // pub name: String,
    // pub version_id: String,
    pub files: Vec<ModrinthIndexFile>,
    pub dependencies: Dependencies,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ModrinthIndexFile {
    pub path: PathBuf,
    // pub hashes: FileHashes,
    pub downloads: Vec<String>,
    // pub file_size: u32, // Maybe this will be needed for progress tracking?
}

// #[derive(Debug, Deserialize, Clone)]
// pub struct FileHashes {
//     pub sha1: String,
//     pub sha512: String,
// }

async fn unzip_modpack(
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

async fn read_modrinth_index(
    modpack_path: &Path,
) -> Result<ModrinthIndex, BackendError> {
    let bytes = tokio::fs::read(modpack_path.join(MODRINTH_INDEX_NAME)).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn copy_overrides(
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

async fn download_modpack_files(
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

pub async fn download_modrinth_modpack<'a>(
    instance_manager: &mut InstanceManager<'a>,
    slug: &str,
    version: &str,
) -> Result<(), BackendError> {
    let project = get_modrinth_project(instance_manager.requester(), slug, version).await?;

    let tmp_dir = TempDir::new(slug)?;
    let zip_path = tmp_dir.path().join(project.id);

    instance_manager
        .requester()
        .builder()
        .download_to(&project.files[0].url, &zip_path)
        .await?;

    unzip_modpack(&zip_path, &tmp_dir.path()).await?;

    let index = read_modrinth_index(&tmp_dir.path()).await?;
    let (mod_loader, mod_loader_version) = index.dependencies.loader.get_loader_info();
    let mc_version = index.dependencies.minecraft;

    instance_manager
        .create_instance(
            slug.to_string(),
            &mc_version,
            mod_loader,
            Some(mod_loader_version.to_string()),
        )
        .await?;

    download_modpack_files(
        instance_manager.requester(),
        &instance_manager.instance_dir(slug),
        index.files,
    )
    .await?;

    copy_overrides(tmp_dir.path(), &instance_manager.instance_dir(slug))?;

    Ok(())
}
