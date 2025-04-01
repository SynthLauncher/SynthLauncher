use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Seek, SeekFrom},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use velcro::hash_map_from;

use super::java::JavaInstallation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config(HashMap<String, String>);

impl Config {
    fn create_default() -> Result<Self, std::io::Error> {
        let java = JavaInstallation::newest();

        Ok(Self(hash_map_from! {
            "min_ram": "512",
            "max_ram": "2048",
            "auth_player_name": "stierprogrammer",
            "auth_access_token": "0",
            "java": java.path.to_string_lossy()
        }))
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
            let config = Self::create_default()?;
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

    pub fn into_mut(self, path: &Path) -> ConfigMut {
        ConfigMut::new(self, path)
    }
}

#[derive(Debug)]
pub struct ConfigMut {
    config: Config,
    fd: File,
}

impl ConfigMut {
    pub fn new(config: Config, path: &Path) -> Self {
        let fd = File::options().write(true).create(true).open(path).unwrap();
        Self { config, fd }
    }

    pub fn set(&mut self, entry: &str, value: String) {
        self.config.0.insert(entry.to_string(), value);
    }

    pub fn remove(&mut self, entry: &str) {
        self.config.0.remove(entry);
    }

    pub fn save(&mut self) {
        self.fd.set_len(0).unwrap();
        self.fd.seek(SeekFrom::Start(0)).unwrap();
        serde_json::to_writer_pretty(&self.fd, &self.config).unwrap();
    }
}

impl Drop for ConfigMut {
    fn drop(&mut self) {
        self.save();
    }
}

impl Deref for ConfigMut {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl DerefMut for ConfigMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}
