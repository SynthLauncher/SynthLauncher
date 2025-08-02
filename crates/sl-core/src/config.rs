use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sl_java_manager::jre_manifest::JreManifest;
use sl_java_manager::{jre_manifest::installer::download_jre_manifest_version, JAVA_BINARY};
use sl_meta::minecraft::loaders::vanilla::{JavaComponent, JavaVersion};
use sl_utils::errors::BackendError;
use sl_utils::requester::Requester;
use sl_utils::wlog;

use crate::instances::InstanceManager;

/// Defines the config file name, relative to the launcher directory and the instance directory.
pub const CONFIG_FILE_NAME: &str = "config.toml";

const fn default_min_memory() -> usize {
    1024
}

const fn default_max_memory() -> usize {
    2048
}

async fn try_get_java_path_or_fetch(
    requester: &Requester,
    jre_manifest: &JreManifest,
    javas_dir: &Path,
    component: &JavaComponent,
) -> Result<PathBuf, BackendError> {
    let java_path = javas_dir.join(component.to_string());

    if !java_path.exists() {
        download_jre_manifest_version(requester, jre_manifest, &javas_dir, component).await?;
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
                    wlog!(
                        "Configured Java path {:?} does not exist, falling back to /usr/bin/java",
                        self.path
                    );
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
                            wlog!(
                                "Configured Java path {:?} does not exist, falling back to {}",
                                self.path,
                                path
                            );
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

/// Returns the default launcher directory path for the current platform.
fn default_launcher_dir() -> PathBuf {
    use std::env;

    #[cfg(target_os = "windows")]
    {
        env::var("APPDATA")
            .map(|appdata| PathBuf::from(appdata).join("SynthLauncher"))
            .expect("%APPDATA% not found")
    }

    #[cfg(target_os = "macos")]
    {
        env::var("HOME")
            .map(|home| {
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("SynthLauncher")
            })
            .expect("$HOME not found")
    }

    #[cfg(target_os = "linux")]
    {
        env::var("HOME")
            .map(|home| PathBuf::from(home).join(".synthlauncher"))
            .expect("$HOME not found")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstanceConfig {
    #[serde(default)]
    pub minecraft: MinecraftConfig,
    pub java: JavaConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LauncherConfig {
    #[serde(flatten)]
    pub instance: InstanceConfig,
}

/// Gets the [`LauncherConfig`] from the given path, only reads doesn't deserialize.
pub(crate) fn get_launcher_config(
    launcher_config_path: &Path,
) -> Result<config::Config, config::ConfigError> {
    let mut config_builder = config::Config::builder();

    if launcher_config_path.exists() {
        config_builder = config_builder.add_source(config::File::from(launcher_config_path));
    }

    let config = config_builder.build()?;
    Ok(config)
}

async fn get_instance_config(
    man: &InstanceManager<'_>,
    instance_local_config_path: &Path,
    java_component: &JavaComponent,
    resolve_against_config: &config::Config,
) -> Result<config::Config, BackendError> {
    let mut config_builder = config::Config::builder()
        .set_default(
            "java.path",
            try_get_java_path_or_fetch(
                man.requester(),
                man.jre_manifest().await,
                man.javas_path(),
                java_component,
            )
            .await?
            .to_str()
            .expect("java path isn't valid UTF-8"),
        )
        .expect("failed to set default java path")
        .add_source(resolve_against_config.clone());

    if instance_local_config_path.exists() {
        config_builder = config_builder.add_source(config::File::from(instance_local_config_path));
    }

    let config = config_builder.build().expect("failed to get config");

    Ok(config)
}

/// Reads the instance configuration from the given directory.
/// has a default state if it doesn't exist anywhere
///
/// TODO: THIS SHOULD ONLY BE USED TO LOAD THE FUNCTION, THE RESULTS OF THIS COULD COME FROM DIFFERENT SOURCES,
/// Implement a method to edit the configuration for an instance and also globally
pub(crate) async fn read_instance_config_against(
    man: &InstanceManager<'_>,
    instance_directory: &Path,
    java_version: &Option<JavaVersion>,
    resolve_against_config: &config::Config,
) -> Result<InstanceConfig, BackendError> {
    let instance_local_config_path = instance_directory.join(CONFIG_FILE_NAME);

    // Temporary workaround
    // TODO: Change
    let component = &java_version
        .as_ref()
        .unwrap_or(&JavaVersion {
            component: JavaComponent::JreLegacy,
            major_version: 8,
        })
        .component;

    get_instance_config(
        man,
        &instance_local_config_path,
        component,
        resolve_against_config,
    )
    .await
    .map(|con| {
        con.try_deserialize::<InstanceConfig>()
            .expect("failed to deserialize config")
    })
}
