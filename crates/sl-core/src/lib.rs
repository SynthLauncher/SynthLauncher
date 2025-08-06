use sl_meta::minecraft::{Arch, OsName};

pub mod accounts;
pub mod config;
pub mod environment;
pub mod instances;
pub(crate) mod java;
pub mod loaders;
pub(crate) mod minecraft;

/// The OS this is compiled to
pub const OS: OsName = if cfg!(target_os = "windows") {
    OsName::Windows
} else if cfg!(target_os = "linux") {
    OsName::Linux
} else if cfg!(target_os = "macos") {
    OsName::Osx
} else {
    panic!("Unsupported OS!")
};

/// The architecture this is compiled to
pub const ARCH: Arch = if cfg!(target_arch = "x86") {
    Arch::X86
} else if cfg!(target_arch = "x86_64") {
    Arch::X86_64
} else if cfg!(target_arch = "aarch64") {
    Arch::ARM64
} else {
    panic!("Unsupported Arch!")
};

// lazy_static! {
//     pub static ref LAUNCHER_DIR: PathBuf = get_launcher_dir();
//     pub static ref ASSETS_DIR: PathBuf = LAUNCHER_DIR.join("assets");
//     pub static ref LIBS_DIR: PathBuf = LAUNCHER_DIR.join("libs");
//     pub static ref INSTANCES_DIR: PathBuf = LAUNCHER_DIR.join("instances");
//     pub static ref VERSIONS_DIR: PathBuf = LAUNCHER_DIR.join("versions");
//     pub static ref JAVAS_DIR: PathBuf = LAUNCHER_DIR.join("javas");
//     pub static ref ADDONS_DIR: PathBuf = LAUNCHER_DIR.join("addons");
//     pub static ref JRE_MANIFEST_PATH: PathBuf = LAUNCHER_DIR.join("jre_manifest.json");
//     pub static ref PROFILES_PATH: PathBuf = LAUNCHER_DIR.join("profiles.json");
//     pub static ref JRE_MANIFEST: JreManifest = read_jre_manifest();
//     pub static ref VERSION_MANIFEST: VersionManifest = read_version_manifest();
//     pub static ref REQUESTER: Requester = Requester::new();
// }
