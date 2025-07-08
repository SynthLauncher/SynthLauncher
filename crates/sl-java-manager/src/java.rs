use std::{
    path::{Path, PathBuf},
    process::Command,
};

use regex::Regex;
use sl_utils::utils::errors::BackendError;
use which::which;

use crate::{JAVA_BINARY, SEPARATOR};

#[derive(Debug)]
pub struct JavaInstallation {
    pub version: String,
    pub path: PathBuf,
}

impl JavaInstallation {
    pub fn extract_java_version(java_path: &Path) -> Result<Option<String>, BackendError> {
        let output = Command::new(java_path).arg("-version").output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        let re = Regex::new(r#"version\s+"([^"]+)""#)?;

        Ok(re
            .captures(&stderr)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string())))
    }

    fn get_java_home() -> Option<PathBuf> {
        std::env::var("JAVA_HOME")
            .ok()
            .map(|java_home| Path::new(&java_home).join("bin").join(JAVA_BINARY))
            .filter(|path| path.is_file())
    }

    fn get_installation_path_from_path() -> Option<PathBuf> {
        which("java").ok()
    }

    fn get_installation_paths_from_path() -> Vec<PathBuf> {
        if let Ok(path) = std::env::var("PATH") {
            return path
                .split(SEPARATOR)
                .map(PathBuf::from)
                .filter_map(|dir| {
                    let file = dir.join(JAVA_BINARY);
                    if file.is_file() { Some(file) } else { None }
                })
                .collect();
        }

        Vec::new()
    }

    fn get_installation_paths_from_dirs<P: AsRef<Path>>(dirs: &[P]) -> Vec<PathBuf> {
        let mut java_paths = Vec::new();

        for dir in dirs {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let java_path = entry.path().join("bin").join(JAVA_BINARY);

                    if java_path.exists() {
                        java_paths.push(java_path);
                    }
                }
            }
        }

        java_paths
    }

    #[cfg(windows)]
    fn get_common_installations() -> Vec<PathBuf> {
        let system_drive = env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
        let mut drive_path = PathBuf::from(&system_drive);
        if drive_path.as_os_str().to_string_lossy().ends_with(':') {
            drive_path.push("\\");
        }

        let common_paths = vec![
            drive_path.join("Program Files").join("Java"),
            drive_path.join("Program Files (x86)").join("Java"),
        ];

        Self::get_installation_paths_from_dirs(&common_paths)
    }

    #[cfg(target_os = "macos")]
    fn get_common_installations() -> Vec<PathBuf> {
        let mut java_paths = Vec::new();

        // Standard Linux/Unix JVM locations
        let common_paths = vec![
            Path::new("/usr/lib/jvm"),
            Path::new("/usr/lib64/jvm"),
            Path::new("/usr/lib32/jvm"),
        ];
        java_paths.extend(Self::get_installation_paths_from_dirs(&common_paths));

        // macOS JavaVirtualMachines
        let jvm_dir = Path::new("/Library/Java/JavaVirtualMachines");
        if let Ok(entries) = fs::read_dir(jvm_dir) {
            for entry in entries.flatten() {
                let java_path = entry
                    .path()
                    .join("Contents")
                    .join("Home")
                    .join("bin")
                    .join(JAVA_BINARY);
                if java_path.exists() {
                    java_paths.push(java_path);
                }
            }
        }

        java_paths
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    fn get_common_installations() -> Vec<PathBuf> {
        let common_paths = vec![
            Path::new("/usr/lib/jvm"),
            Path::new("/usr/lib64/jvm"),
            Path::new("/usr/lib32/jvm"),
        ];
        Self::get_installation_paths_from_dirs(&common_paths)
    }

    pub fn get_all_java_installations() -> Vec<Self> {
        let mut installation_paths = Vec::new();
        let mut java_installations = Vec::new();

        installation_paths.extend(Self::get_java_home());
        installation_paths.extend(Self::get_installation_path_from_path());
        installation_paths.extend(Self::get_installation_paths_from_path());
        installation_paths.extend(Self::get_common_installations());

        installation_paths.sort();
        installation_paths.dedup();

        for path in installation_paths {
            if let Ok(Some(ver)) = Self::extract_java_version(&path) {
                java_installations.push(Self {
                    path: path,
                    version: ver,
                });
            }
        }

        java_installations
    }
}
