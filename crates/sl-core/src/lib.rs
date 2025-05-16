use std::path::PathBuf;

use lazy_static::lazy_static;
use config::config_launcher_dir;
use json::manifest::manifest_read;
use sl_meta::json::{Arch, OsName};
use sl_meta::json::version_manifest::VersionManifest;
use sl_meta::json::vanilla::Client;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::sync::Arc;

pub mod auth;
pub mod config;
pub mod installations;
pub mod json;

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
    pub static ref INSTALLATIONS_DIR: PathBuf = LAUNCHER_DIR.join("installations");
    pub static ref INSTALLATIONS_PATH: PathBuf = LAUNCHER_DIR.join("installations.json");
    pub static ref MANIFEST_PATH: PathBuf = LAUNCHER_DIR.join("version_manifest.json");
    pub static ref MANIFEST: VersionManifest = manifest_read();
    pub static ref TEMP_CLIENT: Lazy<Arc<Mutex<Option<Client>>>> = Lazy::new(|| {
        Arc::new(Mutex::new(None))
    });
}
