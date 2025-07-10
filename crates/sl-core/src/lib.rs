use launcher::java::read_jre_manifest;
use lazy_static::lazy_static;
use minecraft::version_manifest::read_version_manifest;
use sl_java_manager::jre_manifest::JreManifest;
use sl_meta::minecraft::{version_manifest::VersionManifest, Arch, OsName};
use std::path::PathBuf;

use crate::launcher::get_launcher_dir;

pub mod launcher;
pub mod loaders;
pub mod minecraft;

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
    pub static ref LAUNCHER_DIR: PathBuf = get_launcher_dir();
    pub static ref ASSETS_DIR: PathBuf = LAUNCHER_DIR.join("assets");
    pub static ref LIBS_DIR: PathBuf = LAUNCHER_DIR.join("libs");
    pub static ref INSTANCES_DIR: PathBuf = LAUNCHER_DIR.join("instances");
    pub static ref VERSIONS_DIR: PathBuf = LAUNCHER_DIR.join("versions");
    pub static ref JAVAS_DIR: PathBuf = LAUNCHER_DIR.join("javas");
    pub static ref ADDONS_DIR: PathBuf = LAUNCHER_DIR.join("addons");

    pub static ref INSTANCES_PATH: PathBuf = LAUNCHER_DIR.join("instances.json");
    pub static ref VERSION_MANIFEST_PATH: PathBuf = LAUNCHER_DIR.join("version_manifest.json");
    pub static ref JRE_MANIFEST_PATH: PathBuf = LAUNCHER_DIR.join("jre_manifest.json");
    pub static ref PROFILES_PATH: PathBuf = LAUNCHER_DIR.join("profiles.json");

    pub static ref JRE_MANIFEST: JreManifest = read_jre_manifest();
    pub static ref VERSION_MANIFEST: VersionManifest = read_version_manifest();
    pub static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}
