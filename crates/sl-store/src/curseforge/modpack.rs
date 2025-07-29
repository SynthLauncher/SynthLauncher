use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;
use sl_utils::errors::BackendError;
use zip::ZipArchive;

const MODPACK_MANIFEST_NAME: &str = "manifest.json";

#[derive(Debug, Deserialize)]
pub struct ModLoader {
    pub id: String,
    pub primary: bool
}

#[derive(Debug, Deserialize)]
pub struct Minecraft {
    pub version: String,
    pub mod_loaders: Vec<ModLoader>
}

#[derive(Debug, Deserialize)]
pub struct ModpackFile {
    pub project_id: u32,
    pub file_id: u32,
    pub required: bool
}

#[derive(Debug, Deserialize)]
pub struct ModpackManifest {
    pub minecraft: Minecraft,
    pub name: String,
    pub author: String,
    pub files: Vec<ModpackFile>
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
