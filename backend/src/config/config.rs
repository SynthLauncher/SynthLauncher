use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use semver::Version;
use serde::{Deserialize, Serialize};
use velcro::hash_map_from;

use crate::utils::errors::BackendError;

use super::java::JavaInstallation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config(HashMap<String, String>);

// TODO: Remove java version from default and write a function to handle correct java versions for each installations!
impl Config {
    fn create_default_global() -> Result<Self, std::io::Error> {
        Ok(Self(hash_map_from! {
            "auth_player_name": "synther",
            "auth_access_token": "0",
        }))
    }

    // TODO: Finish this
    #[warn(dead_code)]
    fn create_config(version: String) -> Result<JavaInstallation, BackendError> {
        let parsed_version = Version::parse(&version).unwrap();
        let javas = JavaInstallation::get_installations().unwrap();

        if parsed_version <= Version::parse("1.16").unwrap() {
            for java in javas {
                if JavaInstallation::extract_java_version(&java.version.as_str()).unwrap() == 8 {
                    return Ok(java);
                }
            }
        }

        return Err(BackendError::JavaVersionNotFound);
    }
}

#[cfg(test)]
mod tests {
    use crate::config::java::JavaInstallation;

    #[test] 
    fn test() {
        let javas = JavaInstallation::get_installations().unwrap();
        
        for java in javas {
            println!("{}\n", JavaInstallation::extract_java_version(&java.version.as_str()).unwrap());
        }
    }
}

impl Config {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self(map)
    }

    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    fn global_config_path(launcher_root: &Path) -> PathBuf {
        launcher_root.join("config.json")
    }

    pub fn read_global(launcher_root: &Path) -> Result<Self, std::io::Error> {
        let path = Self::global_config_path(launcher_root);

        let config = if !path.exists() {
            let config = Self::create_default_global()?;
            let file = File::create(path)?;
            serde_json::to_writer_pretty(file, &config).unwrap();
            config
        } else {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };

        Ok(config)
    }

    pub fn get(&self, entry: &str) -> Option<&str> {
        self.0.get(entry).map(|x| x.as_str())
    }

    pub fn merge(self, mut other: Self) -> Self {
        other.0.extend(self.0);
        other
    }
}
