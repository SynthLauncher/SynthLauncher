use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Error};
use itertools::Itertools;
use regex::Regex;

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

    fn find_java_home() -> Option<Self> {
        if let Ok(java_home) = env::var("JAVA_HOME") {
            let java_path = Path::new(&java_home).join("bin").join(if cfg!(windows) {
                "java.exe"
            } else {
                "java"
            });

            if java_path.exists() {
                return match Self::from_path(&java_path) {
                    Ok(java_installation) => Some(java_installation),
                    Err(_) => None,
                };
            }
        }

        None
    }
}
