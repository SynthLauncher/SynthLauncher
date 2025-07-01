use std::{
    path::{Path, PathBuf},
    process::Command,
};

use regex::Regex;
use sl_utils::utils::errors::BackendError;
use which::which;

#[cfg(windows)]
const JAVA_BINARY: &str = "java.exe";
#[cfg(not(windows))]
const JAVA_BINARY: &str = "java";

#[cfg(windows)]
const SEPARATOR: &str = ";";
#[cfg(not(windows))]
const SEPARATOR: &str = ":";

#[derive(Debug)]
pub struct JavaInstallation {
    pub path: PathBuf,
    pub version: String,
}

impl JavaInstallation {
    pub fn extract_java_version(java_path: &Path) -> Result<Option<String>, BackendError> {
        let output = Command::new(java_path).arg("-version").output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        let re = Regex::new(r#"version\s+"([\d._""#)?;
        Ok(re
            .captures(&stderr)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string())))
    }

    pub fn get_java_home() -> Option<PathBuf> {
        std::env::var("JAVA_HOME")
            .ok()
            .map(|java_home| Path::new(&java_home).join("bin").join(JAVA_BINARY))
            .filter(|path| path.is_file())
    }

    pub fn get_java_from_path() -> Option<PathBuf> {
        which("java").ok()
    }

    pub fn get_javas_from_path() -> Vec<PathBuf> {
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

    pub fn get_javas_from_dir() -> Vec<PathBuf> {
        Vec::new()
    }
}
