use std::{fs::File, io::Read, path::Path};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha1::Digest;
use sha2::Sha512;
use sl_utils::utils::errors::BackendError;
use zip::ZipArchive;

use crate::launcher::instances::metadata::ModLoader;

#[derive(Debug, Deserialize, Serialize)]
pub struct ModMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mod {
    pub metadata: ModMetadata,
    pub file_name: String,
    pub sha512: String,
    pub icon: Option<String>,
}

pub fn get_mod_metadata(
    instance_mod_loader: &ModLoader,
    mod_path: &Path,
) -> Result<Option<Mod>, BackendError> {
    let mut file = File::open(mod_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let sha512 = hex::encode(Sha512::digest(&buf));

    let meta_entry_name = match instance_mod_loader {
        ModLoader::Fabric => "fabric.mod.json",
        ModLoader::Quilt => "quilt.mod.json",
        ModLoader::Forge => unimplemented!("Forge metadata parsing"),
        ModLoader::NeoForge => unimplemented!("NeoForge parsing"),
        ModLoader::Vanilla => return Ok(None),
    };

    let metadata = {
        let mut archive = ZipArchive::new(File::open(mod_path)?)?;
        let mut entry = match archive.by_name(meta_entry_name) {
            Ok(e) => e,
            Err(_) => return Ok(None),
        };
        let mut json_str = String::new();
        entry.read_to_string(&mut json_str)?;
        serde_json::from_str::<ModMetadata>(&json_str)?
    };

    let icon = {
        let mut archive = ZipArchive::new(File::open(mod_path)?)?;
        (0..archive.len()).find_map(|i| {
            let mut entry = archive.by_index(i).ok()?;
            let n = entry.name();
            if !n.contains('/') && n.to_lowercase().ends_with(".png") {
                let mut buf2 = Vec::new();
                entry.read_to_end(&mut buf2).ok()?;
                return Some(general_purpose::STANDARD.encode(buf2));
            }
            None
        })
    };

    let file_name = mod_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();

    Ok(Some(Mod {
        metadata,
        file_name,
        sha512,
        icon,
    }))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinecraftWorldMetadata {
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScreenshotMetadata {
    pub name: String,
    pub screenshot: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameInfo {
    pub worlds: Vec<MinecraftWorldMetadata>,
    pub screenshots: Vec<ScreenshotMetadata>,
    pub mods: Vec<Mod>,
}
