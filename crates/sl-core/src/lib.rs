use sl_meta::minecraft::{Arch, OsName};

pub mod accounts;
pub mod config;
pub mod environment;
pub mod instances;
pub(crate) mod java;
pub mod loaders;
pub(crate) mod minecraft;
pub use sl_utils;

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
