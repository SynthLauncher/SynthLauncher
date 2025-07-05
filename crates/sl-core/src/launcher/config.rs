use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sl_meta::java::jre_manifest::JreManifestDownloadType;
use sl_utils::utils::errors::BackendError;

use crate::{
    java::jre_manifest::download_jre_manifest_version,
    JAVAS_DIR, LAUNCHER_DIR,
};

/// Defines the config file name, relative to the launcher directory and the instance directory.
pub const CONFIG_FILE_NAME: &str = "config.toml";

fn launcher_config_name() -> String {
    LAUNCHER_DIR
        .join(CONFIG_FILE_NAME)
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
pub struct JavaConfig {
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
pub struct InstanceConfig {
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
    let instance_local_config_path = instance_directory.join(CONFIG_FILE_NAME);
    get_instance_config(&instance_local_config_path, java_version)
        .await
        .map(|con| {
            con.try_deserialize::<InstanceConfig>()
                .expect("failed to deserialize config")
        })
}
