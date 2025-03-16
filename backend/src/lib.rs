use std::path::PathBuf;

use config::app::config_launcher_dir;
use lazy_static::lazy_static;
use synthlauncher_meta::json::platform::{Arch, OsName};

pub mod utils;
pub mod config;
pub mod installation;
pub mod json;

pub const OS: OsName = if cfg!(target_os = "windows") {
    OsName::Windows
} else if cfg!(target_os = "linux") {
    OsName::Linux
} else if cfg!(target_os = "macos") {
    OsName::Osx
} else {
    panic!("Unknown OS!")
};

pub const ARCH: Arch = if cfg!(target_arch = "x86") {
    Arch::X86
} else if cfg!(target_arch = "x86_64") {
    Arch::X86_64
} else if cfg!(target_arch = "aarch64") {
    Arch::ARM64
} else {
    panic!("Unknown Arch")
};

lazy_static! {
    #[derive(Debug)]
    pub static ref LAUNCHER_DIR: PathBuf = config_launcher_dir();
    pub static ref INSTALLATIONS_DIR: PathBuf = LAUNCHER_DIR.join("installations");
    pub static ref ASSETS_DIR: PathBuf = LAUNCHER_DIR.join("assets");
    pub static ref LIBS_DIR: PathBuf = LAUNCHER_DIR.join("libs");
}
