pub mod java;
pub mod jre_manifest;

#[derive(Debug)]
enum Platform {
    Linux,
    LinuxI386,
    MacOs,
    MacOsArm64,
    WindowsArm64,
    WindowsX86,
    WindowsX64,
}

impl Platform {
    fn detect() -> Self {
        use Platform::*;

        match (std::env::consts::OS, std::env::consts::ARCH) {
            ("linux", "x86_64") => Linux,
            ("linux", "x86") | ("linux", "i386") => LinuxI386,
            ("macos", "x86_64") => MacOs,
            ("macos", "aarch64") => MacOsArm64,
            ("windows", "aarch64") => WindowsArm64,
            ("windows", "x86") | ("windows", "i386") => WindowsX86,
            ("windows", "x86_64") => WindowsX64,
            _ => panic!("Unsupported platform!"),
        }
    }
}

pub const JAVA_BINARY: &str = if cfg!(target_os = "windows") {
    "java.exe"
} else {
    "java"
};

pub const MULTI_PATH_SEPARATOR: &str = if cfg!(target_os = "windows") {
    ";"
} else {
    ":"
};
