use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sl_java_manager::{jre_manifest::installer::download_jre_manifest_version, JAVA_BINARY};
use sl_meta::{minecraft::loaders::vanilla::JavaComponent};
use sl_utils::errors::BackendError;
use sl_utils::wlog;

use crate::{
    JAVAS_DIR, JRE_MANIFEST, LAUNCHER_DIR, REQUESTER
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

async fn default_java_path(component: &JavaComponent) -> Result<PathBuf, BackendError> {
    let java_path = JAVAS_DIR.join(component.to_string());

    if !java_path.exists() {
        download_jre_manifest_version(
            &REQUESTER,
            &JRE_MANIFEST,
            &JAVAS_DIR,
            component
        ).await?;
    }

    Ok(java_path.join("bin").join(JAVA_BINARY))
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
        // If the configured path exists, use it
        if self.path.exists() {
            &self.path
        } else {
            #[cfg(target_os = "macos")]
            {
                use std::path::Path;
                // Fallback to /usr/bin/java if available
                static FALLBACK: &str = "/usr/bin/java";
                if Path::new(FALLBACK).exists() {
                    wlog!("Configured Java path {:?} does not exist, falling back to /usr/bin/java", self.path);
                    return Path::new(FALLBACK);
                }
            }
            #[cfg(not(target_os = "macos"))]
            {
                use std::process::Command;
                if let Ok(output) = Command::new("which").arg("java").output() {
                    if output.status.success() {
                        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if !path.is_empty() && std::path::Path::new(&path).exists() {
                            wlog!("Configured Java path {:?} does not exist, falling back to {}", self.path, path);
                            return std::path::Path::new(Box::leak(path.into_boxed_str()));
                        }
                    }
                }
            }
            // If all else fails, return the original (will error later)
            &self.path
        }
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
    java_component: &JavaComponent,
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
    java_version: &JavaComponent,
) -> Result<InstanceConfig, BackendError> {
    let instance_local_config_path = instance_directory.join(CONFIG_FILE_NAME);
    get_instance_config(&instance_local_config_path, java_version)
        .await
        .map(|con| {
            con.try_deserialize::<InstanceConfig>()
                .expect("failed to deserialize config")
        })
}
