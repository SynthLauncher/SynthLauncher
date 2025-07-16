use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha1::Digest;
use sha2::Sha512;
use sl_utils::{errors::BackendError};
use std::{fs::File, io::Read, path::Path};
use zip::ZipArchive;

use crate::{launcher::instances::instance_metadata::ModLoader, INSTANCES_DIR};

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
pub struct ScreenshotMetadata {
    pub name: String,
    pub screenshot: String,
}

pub fn get_screenshot_metadata(screenshot_path: &Path) -> Result<ScreenshotMetadata, BackendError> {
    let screenshot_name = screenshot_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();

    let image_bytes = std::fs::read(screenshot_path)?;

    let image_base64 = general_purpose::STANDARD.encode(&image_bytes);

    Ok(ScreenshotMetadata {
        name: screenshot_name,
        screenshot: image_base64,
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinecraftWorldMetadata {
    pub name: String,
    pub icon: String,
}

pub fn get_minecraft_world_metadata(
    world_folder_path: &Path,
) -> Result<MinecraftWorldMetadata, BackendError> {
    let world_name = world_folder_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();

    let icon_path = world_folder_path.join("icon.png");
    let image_bytes = std::fs::read(icon_path)?;

    let image_base64 = general_purpose::STANDARD.encode(&image_bytes);
    Ok(MinecraftWorldMetadata { name:  world_name, icon: image_base64 })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameInfo {
    pub worlds: Vec<MinecraftWorldMetadata>,
    pub screenshots: Vec<ScreenshotMetadata>,
}

pub fn get_game_info(instance_name: &str) -> Result<GameInfo, BackendError> {
    let instance_path  = &*INSTANCES_DIR.join(instance_name);
    let saves_path = instance_path.join("saves");
    let mut worlds = Vec::new();

    if saves_path.exists() {
        for entry in std::fs::read_dir(&saves_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                match get_minecraft_world_metadata(&path) {
                    Ok(world) => worlds.push(world),
                    Err(e) => {
                        eprintln!("Failed to load world metadata for {:?}: {}", path, e);
                    }
                }
            }
        }
    }

    let screenshots_path = instance_path.join("screenshots");
    let mut screenshots = Vec::new();

    if screenshots_path.exists() {
        for entry in std::fs::read_dir(&screenshots_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()).unwrap_or("") == "png" {
                match get_screenshot_metadata(&path) {
                    Ok(screenshot) => screenshots.push(screenshot),
                    Err(e) => {
                        eprintln!("Failed to load screenshot metadata for {:?}: {}", path, e);
                    }
                }
            }
        }
    }

    Ok(GameInfo {
        worlds,
        screenshots,
    })
}