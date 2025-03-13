use json::platform::{Arch, OsType};

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
