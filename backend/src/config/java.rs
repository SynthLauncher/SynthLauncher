use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Error};
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};
use which::which;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JavaInstallation {
    pub version: String,
    pub path: PathBuf,
}

impl JavaInstallation {
    pub fn new(version: String, path: PathBuf) -> Self {
        Self { version, path }
    }

    fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
        let parse_version = |v: &str| {
            v.split(|c| c == '.' || c == '_')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect_vec()
        };

        let a_parts = parse_version(a);
        let b_parts = parse_version(b);

        a_parts.cmp(&b_parts)
    }

    fn get_java_version(java_path: &Path) -> Result<String, anyhow::Error> {
        let output = Command::new(java_path)
            .arg("-version")
            .output()
            .context("Failed to execute java command")?;

        let output_str = String::from_utf8_lossy(&output.stderr);
        let regx = Regex::new(r#"version\s+\"(\d+\.\d+\.\d+)[_-]?(\d+)?\""#)?;

        if let Some(caps) = regx.captures(&output_str) {
            let mut version = caps[1].to_string();
            if let Some(update) = caps.get(2) {
                version.push('_');
                version.push_str(update.as_str());
            }

            return Ok(version);
        }

        Err(anyhow::anyhow!("Version string not found!"))
    }

    fn from_path(path: &Path) -> Result<Self, Error> {
        let version = Self::get_java_version(path)?;
        Ok(Self::new(version, path.to_path_buf()))
    }

    fn find_java_home() -> Result<Option<Self>, Error> {
        if let Ok(java_home) = env::var("JAVA_HOME") {
            let java_path = Path::new(&java_home).join("bin").join(if cfg!(windows) {
                "java.exe"
            } else {
                "java"
            });

            if java_path.exists() {
                return Ok(Some(Self::from_path(&java_path)?));
            }
        }

        Ok(None)
    }

    fn find_in_path() -> Result<Vec<Self>, Error> {
        let mut installations = Vec::new();
        if let Ok(path) = which("java") {
            if let Ok(installation) = Self::from_path(&path) {
                installations.push(installation);
            }
        }

        Ok(installations)
    }

    fn search_java_dirs(paths: &[&std::path::Path]) -> Result<Vec<Self>, Error> {
        let mut installations = Vec::new();
        for path in paths {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let java_path = entry.path().join("bin").join(if cfg!(windows) {
                        "java.exe"
                    } else {
                        "java"
                    });

                    if java_path.exists() {
                        if let Ok(installation) = Self::from_path(&java_path) {
                            installations.push(installation);
                        }
                    }
                }
            }
        }

        Ok(installations)
    }

    #[cfg(target_os = "windows")]
    fn find_common_installations() -> Result<Vec<Self>, Error> {
        let system_drive = env::var("SystemDrive").unwrap_or_else(|_| "C:".to_string());
        let mut drive_path = PathBuf::from(&system_drive);
        if drive_path.as_os_str().to_string_lossy().ends_with(':') {
            drive_path.push("\\");
        }

        let common_paths = vec![
            drive_path.join("Program Files").join("Java"),
            drive_path.join("Program Files (x86)").join("Java"),
        ];

        Self::search_java_dirs(&common_paths)
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn find_common_installations() -> Result<Vec<Self>, Error> {
        let common_paths = vec![
            Path::new("/usr/lib/jvm"),
            Path::new("/usr/lib64/jvm"),
            Path::new("/usr/lib32/jvm"),
            Path::new("/Library/Java/JavaVirtualMachines"),
        ];

        Self::search_java_dirs(&common_paths)
    }

    pub fn get_installations() -> Result<Vec<Self>, Error> {
        let mut installations = Vec::new();

        installations.extend(Self::find_common_installations()?);
        installations.extend(Self::find_in_path()?);

        if let Some(java_home) = Self::find_java_home()? {
            installations.push(java_home);
        }

        installations.sort_by_cached_key(|i| i.version.clone());
        installations.dedup_by(|a, b| a.path == b.path);
        installations.sort_by(|a, b| Self::compare_versions(&a.version, &b.version));

        Ok(installations)
    }

    pub fn newest() -> JavaInstallation {
        Self::get_installations()
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
    }
}
