pub mod config;

use std::{
    env,
    fs::{self, OpenOptions},
    path::PathBuf,
};

use sl_utils::utils::errors::BackendError;

use crate::{
    json::{jre_manifest::fetch_jre_manifest, version_manifest::fetch_version_manifest},
    ASSETS_DIR, INSTANCES_DIR, INSTANCES_PATH, JAVAS_DIR, LAUNCHER_DIR, LIBS_DIR, PROFILES_PATH,
};

pub fn config_launcher_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return env::var("APPDATA")
            .map(|appdata| PathBuf::from(appdata).join("SynthLauncher"))
            .unwrap_or_else(|_| PathBuf::from("C:\\SynthLauncher"));
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
            .unwrap_or_else(|_| PathBuf::from("/usr/local/synthlauncher"));
    }

    #[cfg(target_os = "linux")]
    {
        return env::var("HOME")
            .map(|home| PathBuf::from(home).join(".synthlauncher"))
            .unwrap_or_else(|_| PathBuf::from("/usr/local/synthlauncher"));
    }
}

pub async fn init_launcher_dir() -> Result<(), BackendError> {
    fs::create_dir_all(&(*LAUNCHER_DIR)).unwrap();
    fs::create_dir_all(&(*LIBS_DIR)).unwrap();
    fs::create_dir_all(&(*ASSETS_DIR)).unwrap();
    fs::create_dir_all(&(*INSTANCES_DIR)).unwrap();
    fs::create_dir_all(&(*JAVAS_DIR)).unwrap();
  
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&*INSTANCES_PATH)?;
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&*PROFILES_PATH)?;

    fetch_version_manifest().await;
    fetch_jre_manifest().await;

    std::env::set_current_dir(&*LAUNCHER_DIR)?;

    Ok(())
}
