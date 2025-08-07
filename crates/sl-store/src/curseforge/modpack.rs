use std::{fs::File, io::BufReader, path::Path, str::FromStr};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::instances::instance_metadata::ModLoader;
use sl_utils::{errors::BackendError, requester::Requester};
use zip::ZipArchive;

use crate::curseforge::api::project::get_curseforge_project_file;

const MODPACK_MANIFEST_NAME: &str = "manifest.json";

#[derive(Debug, Deserialize)]
pub struct CurseforgeModLoader {
    pub id: String,
    pub primary: bool,
}

impl CurseforgeModLoader {
    pub fn extract_mod_loader(&self) -> (ModLoader, &str) {
        let (mod_loader, version) = self.id.rsplit_once("-").expect("ModLoader id must have a hyphen!");
        (ModLoader::from_str(mod_loader).unwrap(), version)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Minecraft {
    pub version: String,
    pub mod_loaders: Vec<CurseforgeModLoader>,
}

#[derive(Debug, Deserialize, Clone)]
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
    let bytes = tokio::fs::read(modpack_path.join(MODPACK_MANIFEST_NAME)).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

async fn download_modpack_file(
    requester: &Requester,
    mods_folder: &Path,
    modpack_file: &ModpackFile,
) -> Result<(), BackendError> {
    let project_file =
        get_curseforge_project_file(requester, modpack_file.project_id, modpack_file.file_id).await?;
    let mod_path = mods_folder.join(project_file.file_name);

    requester
        .builder()
        .download_to(&project_file.download_url, &mod_path)
        .await?;

    Ok(())
}

pub async fn download_modpack_files(
    requester: &Requester,
    instance_path: &Path,
    modpack_files: Vec<ModpackFile>,
) -> Result<(), BackendError> {
    let tasks = FuturesUnordered::new();

    for modpack_file in &modpack_files {
        tasks.push(download_modpack_file(&requester, &instance_path, &modpack_file));
    }

    let futures = tasks.collect::<Vec<_>>().await;
    for result in futures {
        result?;
    }

    Ok(())
}