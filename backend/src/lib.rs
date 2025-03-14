use std::path::PathBuf;

use config::app::config_launcher_dir;
use lazy_static::lazy_static;
use synthlauncher_interface::json::platform::{Arch, OsType};

pub mod config;
pub mod installation;
pub mod json;

pub const OS: OsType = if cfg!(target_os = "windows") {
    OsType::Windows
} else if cfg!(target_os = "linux") {
    OsType::Linux
} else if cfg!(target_os = "macos") {
    OsType::Osx
} else {
    panic!("Unknown OS!")
};

pub const ARCH: Arch = if cfg!(target_arch = "x86") {
    Arch::X86
} else if cfg!(target_arch = "x86_64") {
    Arch::X86_64
} else if cfg!(target_arch = "aarch64") {
    Arch::ARM64
} else if cfg!(target_arch = "arm") {
    Arch::ARM
} else {
    panic!("Unknown Arch")
};

lazy_static! {
    pub static ref LAUNCHER_DIR: PathBuf = config_launcher_dir();
    pub static ref INSTALLATIONS_DIR: PathBuf = LAUNCHER_DIR.join("installations");
    pub static ref ASSETS_DIR: PathBuf = LAUNCHER_DIR.join("assets");
    pub static ref LIBS_DIR: PathBuf = LAUNCHER_DIR.join("libs");
}
