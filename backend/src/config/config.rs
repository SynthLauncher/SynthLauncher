use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use velcro::hash_map_from;

use crate::{utils::errors::BackendError, LAUNCHER_DIR};

use super::java::JavaInstallation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config(HashMap<String, String>);

impl Config {
    fn create_default_global() -> Result<Self, std::io::Error> {
        Ok(Self(hash_map_from! {
            "auth_player_name": "synther",
            "auth_access_token": "0",
        }))
    }

    pub fn create_config(java_version: u16) -> Result<Self, BackendError> {
        let javas = JavaInstallation::get_installations().unwrap();

        for java in javas {
            if JavaInstallation::extract_java_version(&java.version.as_str()).unwrap() == java_version {  
                return Ok(Self(hash_map_from! {
                    "java": java.path.to_string_lossy()
                }));
            }
        }

        Err(BackendError::JavaVersionNotFound)
    }
}

impl Config {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self(map)
    }

    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    fn global_config_path() -> PathBuf {
        LAUNCHER_DIR.join("config.json")
    }

    pub fn read_global() -> Result<Self, std::io::Error> {
        let path = Self::global_config_path();

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
