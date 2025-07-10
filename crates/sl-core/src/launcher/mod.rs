use std::path::PathBuf;
use std::{env, fs::OpenOptions};

use sl_utils::dlog;
use sl_utils::errors::BackendError;
use sl_utils::log::set_log_file;

use crate::launcher::java::fetch_jre_manifest;
use crate::minecraft::version_manifest::fetch_version_manifest;
use crate::{
    ADDONS_DIR, ASSETS_DIR, INSTANCES_DIR, INSTANCES_PATH, JAVAS_DIR, LAUNCHER_DIR, LIBS_DIR,
    PROFILES_PATH,
};

pub mod config;
pub mod instances;
pub mod java;
pub mod player;
pub mod player_profiles;
mod minecraft_version;

pub fn get_launcher_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return env::var("APPDATA")
            .map(|appdata| PathBuf::from(appdata).join("SynthLauncher"))
            .expect("%APPDATA% not found");
    }

    #[cfg(target_os = "macos")]
    {
        return env::var("HOME")
            .map(|home| {
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("SynthLauncher")
            })
            .expect("$HOME not found");
    }

    #[cfg(target_os = "linux")]
    {
        return env::var("HOME")
            .map(|home| PathBuf::from(home).join(".synthlauncher"))
            .expect("$HOME not found");
    }
}

pub async fn init_launcher_dir() -> Result<(), BackendError> {
    for dir in [
        &(*LAUNCHER_DIR),
        &(*LIBS_DIR),
        &(*ASSETS_DIR),
        &(*INSTANCES_DIR),
        &(*JAVAS_DIR),
        &(*ADDONS_DIR),
    ] {
        dlog!("{} dir initialized!", &dir.display());
        tokio::fs::create_dir_all(dir).await?;
    }

    for path in [&(*INSTANCES_PATH), &(*PROFILES_PATH)] {
        dlog!("{} path initialized!", path.display());
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
    }

    dlog!("Fetching version manifest!");
    fetch_version_manifest().await;
    dlog!("Fetched version manifest!");

    dlog!("Fetching JRE manifest!");
    fetch_jre_manifest().await;
    dlog!("Fetched JRE manifest!");

    let log_file_path = LAUNCHER_DIR.join("last_run.log");
    set_log_file(log_file_path);

    std::env::set_current_dir(&*LAUNCHER_DIR)?;

    Ok(())
}
