use std::{
    env,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sl_meta::java::jre_manifest::JreManifestDownloadType;
use sl_utils::{dlog, utils::errors::BackendError};

use crate::{
    java::jre_manifest::{download_jre_manifest_version, fetch_jre_manifest}, minecraft::version_manifest::fetch_version_manifest, ADDONS_DIR, ASSETS_DIR, INSTANCES_DIR, INSTANCES_PATH, JAVAS_DIR, LAUNCHER_DIR, LIBS_DIR, PROFILES_PATH
};

pub fn get_launcher_dir() -> PathBuf {
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
    for dir in [
        &(*LAUNCHER_DIR),
        &(*LIBS_DIR),
        &(*ASSETS_DIR),
        &(*INSTANCES_DIR),
        &(*JAVAS_DIR),
        &(*ADDONS_DIR)
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

    std::env::set_current_dir(&*LAUNCHER_DIR)?;

    Ok(())
}

fn launcher_config_name() -> String {
    LAUNCHER_DIR
        .join("config.toml")
        .to_string_lossy()
        .to_string()
}

const fn default_min_memory() -> usize {
    1024
}

const fn default_max_memory() -> usize {
    2048
}

async fn default_java_path(component: &JreManifestDownloadType) -> Result<PathBuf, BackendError> {
    let java_path = JAVAS_DIR.join(component.to_string());

    let java_binary = if cfg!(target_os = "windows") {
        "java.exe"
    } else {
        "java"
    };

    if !java_path.exists() {
        download_jre_manifest_version(component).await?;
    }

    Ok(java_path.join("bin").join(java_binary))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MinecraftConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct JavaConfig {
    #[serde(default = "default_min_memory")]
    pub min_ram: usize,
    #[serde(default = "default_max_memory")]
    pub max_ram: usize,
    // default is going to be set by another part of the code
    // please give everything a default value somewhere
    pub path: PathBuf,
    pub javac_path: Option<PathBuf>,
}

impl JavaConfig {
    pub fn java(&self) -> &Path {
        &self.path
    }

    pub fn get_javac(&self) -> PathBuf {
        match self.javac_path {
            Some(ref path) => path.clone(),
            None => self.java().with_file_name("javac"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct InstanceConfig {
    #[serde(default)]
    pub minecraft: MinecraftConfig,
    pub java: JavaConfig,
}

async fn get_instance_config(
    instance_local_config_path: &Path,
    java_component: &JreManifestDownloadType,
) -> Result<config::Config, BackendError> {
    let instance_local_config_name = instance_local_config_path
        .to_str()
        .expect("instance local config isn't a Path");

    let launcher_config_name = launcher_config_name();
    let mut config_builder = config::Config::builder()
        .set_default(
            "java.path",
            default_java_path(java_component)
                .await?
                .to_str()
                .expect("java path isn't valid UTF-8"),
        )
        .expect("failed to set default java path");

    if std::fs::exists(&launcher_config_name).is_ok_and(|r| r) {
        config_builder = config_builder.add_source(config::File::with_name(&launcher_config_name));
    }

    if instance_local_config_path.exists() {
        config_builder =
            config_builder.add_source(config::File::with_name(instance_local_config_name));
    }

    let config = config_builder.build().expect("failed to get config");

    Ok(config)
}

/// Reads the instance configuration from the given directory.
/// has a default state if it doesn't exist anywhere
///
/// TODO: THIS SHOULD ONLY BE USED TO LOAD THE FUNCTION, THE RESULTS OF THIS COULD COME FROM DIFFERENT SOURCES,
/// Implement a method to edit the configuration for an instance and also globally
pub(crate) async fn read_instance_config(
    instance_directory: &Path,
    java_version: &JreManifestDownloadType,
) -> Result<InstanceConfig, BackendError> {
    let instance_local_config_path = instance_directory.join("config.toml");
    get_instance_config(&instance_local_config_path, java_version)
        .await
        .map(|con| {
            con.try_deserialize::<InstanceConfig>()
                .expect("failed to deserialize config")
        })
}
