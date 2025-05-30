use std::path::PathBuf;

use config::config_launcher_dir;
use json::jre_manifest::jre_manifest_read;
use json::version_manifest::version_manifest_read;
use lazy_static::lazy_static;
use sl_meta::json::jre_manifest::JreManifest;
use sl_meta::json::version_manifest::VersionManifest;
use sl_meta::json::{Arch, OsName};
use profiles::player::PlayerProfile;

pub mod config;
pub mod instance;
pub mod instances;
pub mod json;
pub mod profiles;

pub const MULTI_PATH_SEPARATOR: &'static str = if cfg!(target_os = "windows") {
    ";"
} else {
    ":"
};

pub const OS: OsName = if cfg!(target_os = "windows") {
    OsName::Windows
} else if cfg!(target_os = "linux") {
    OsName::Linux
} else if cfg!(target_os = "macos") {
    OsName::Osx
} else {
    panic!("Unsupported OS")
};

pub const ARCH: Arch = if cfg!(target_arch = "x86") {
    Arch::X86
} else if cfg!(target_arch = "x86_64") {
    Arch::X86_64
} else if cfg!(target_arch = "aarch64") {
    Arch::ARM64
} else {
    panic!("Unsupported Arch")
};

lazy_static! {
    #[derive(Debug)]
    pub static ref LAUNCHER_DIR: PathBuf = config_launcher_dir();
    pub static ref ASSETS_DIR: PathBuf = LAUNCHER_DIR.join("assets");
    pub static ref LIBS_DIR: PathBuf = LAUNCHER_DIR.join("libs");
    pub static ref INSTANCES_DIR: PathBuf = LAUNCHER_DIR.join("instances");
    pub static ref JAVAS_DIR: PathBuf = LAUNCHER_DIR.join("javas");
    
    pub static ref INSTANCES_PATH: PathBuf = LAUNCHER_DIR.join("instances.json");
    pub static ref VERSION_MANIFEST_PATH: PathBuf = LAUNCHER_DIR.join("version_manifest.json");
    pub static ref JRE_MANIFEST_PATH: PathBuf = LAUNCHER_DIR.join("jre_manifest.json");
    pub static ref PROFILES_PATH: PathBuf = LAUNCHER_DIR.join("profiles.json");

    pub static ref JRE_MANIFEST: JreManifest = jre_manifest_read();
    pub static ref VERSION_MANIFEST: VersionManifest = version_manifest_read();
    pub static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
    pub static ref CURRENT_PROFILE: Option<PlayerProfile> = None;
}
