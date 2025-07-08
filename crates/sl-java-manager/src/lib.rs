pub mod java;
pub mod jre_manifest;

#[derive(Debug)]
pub enum Platform {
    Linux,
    LinuxI386,
    MacOs,
    MacOsArm64,
    WindowsArm64,
    WindowsX86,
    WindowsX64,
}

impl Platform {
    pub fn detect() -> Self {
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

#[cfg(windows)]
pub const JAVA_BINARY: &'static str = "java.exe";
#[cfg(not(windows))]
pub const JAVA_BINARY: &'static str = "java";

#[cfg(windows)]
const SEPARATOR: &str = ";";
#[cfg(not(windows))]
const SEPARATOR: &str = ":";

