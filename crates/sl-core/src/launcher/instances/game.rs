use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha1::Digest;
use sha2::Sha512;
use sl_utils::{errors::BackendError, wlog};
use std::{
    io::{BufReader, Cursor, Read},
    path::Path,
};
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
    let mod_data = std::fs::read(mod_path)?;
    let sha512 = hex::encode(Sha512::digest(&mod_data));

    let meta_entry_name = match instance_mod_loader {
        ModLoader::Fabric => "fabric.mod.json",
        ModLoader::Quilt => "quilt.mod.json",
        ModLoader::Forge => unimplemented!("Forge metadata parsing"),
        ModLoader::NeoForge => unimplemented!("NeoForge parsing"),
        ModLoader::Vanilla => return Ok(None),
    };

    let mut archive = ZipArchive::new(Cursor::new(&mod_data))?;

    let metadata = {
        let entry = match archive.by_name(meta_entry_name) {
            Ok(e) => e,
            Err(_) => return Ok(None),
        };

        let entry_reader = BufReader::new(entry);
        serde_json::from_reader::<_, ModMetadata>(entry_reader)?
    };

    let icon = {
        (0..archive.len()).find_map(|i| {
            let mut entry = archive.by_index(i).ok()?;
            let entry_name = entry.name();

            if !entry_name.contains('/') && entry_name.ends_with(".png") {
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
        .expect("mod path isn't valid utf8")
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
    let image_bytes = std::fs::read(screenshot_path)?;
    let image_base64 = general_purpose::STANDARD.encode(&image_bytes);

    let screenshot_name = screenshot_path
        .file_name()
        .and_then(|s| s.to_str())
        .expect("screenshot path isn't valid utf8")
        .to_string();

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
    let icon_path = world_folder_path.join("icon.png");

    let image_bytes = std::fs::read(icon_path)?;
    let image_base64 = general_purpose::STANDARD.encode(&image_bytes);

    let world_name = world_folder_path
        .file_name()
        .and_then(|s| s.to_str())
        .expect("minecraft save path isn't valid utf8")
        .to_string();
    Ok(MinecraftWorldMetadata {
        name: world_name,
        icon: image_base64,
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameInfo {
    pub worlds: Vec<MinecraftWorldMetadata>,
    pub screenshots: Vec<ScreenshotMetadata>,
}

pub fn get_game_info(instance_name: &str) -> Result<GameInfo, BackendError> {
    /// For each entry in `dir_path`, retrieve it's path, filter it by `filter_f` and get metadata using `get_f(path)`.
    fn get_metadata_of<R, E, F>(
        dir_path: &Path,
        filter_f: impl Fn(&Path) -> bool,
        get_f: F,
    ) -> Result<Vec<R>, E>
    where
        E: std::error::Error + From<std::io::Error>,
        F: Fn(&Path) -> Result<R, E>,
    {
        let read_dir = std::fs::read_dir(dir_path)?;
        let read_dir_iter = read_dir.into_iter();
        let results_iter = read_dir_iter
            .filter_map(|ent| ent.ok())
            .map(|ent| ent.path())
            .filter(|path| filter_f(path))
            .filter_map(|path| match get_f(path.as_path()) {
                Ok(world) => Some(world),
                Err(err) => {
                    wlog!(
                        "failed to read metadata for: {}, err: {err}",
                        path.display()
                    );
                    None
                }
            });

        Ok(results_iter.collect())
    }

    let instance_path = &*INSTANCES_DIR.join(instance_name);
    let saves_path = instance_path.join("saves");

    let worlds = if saves_path.exists() {
        get_metadata_of(
            &saves_path,
            |path| path.is_dir(),
            get_minecraft_world_metadata,
        )?
    } else {
        Vec::new()
    };

    let screenshots_path = instance_path.join("screenshots");
    let screenshots = if screenshots_path.exists() {
        get_metadata_of(
            &screenshots_path,
            |path| {
                path.is_file()
                    && path
                        .extension()
                        .is_some_and(|s| s.as_encoded_bytes() == b"png")
            },
            get_screenshot_metadata,
        )?
    } else {
        Vec::new()
    };

    Ok(GameInfo {
        worlds,
        screenshots,
    })
}
