use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::{INSTALLATIONS_DIR, utils::errors::BackendError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Installation {
    pub version: String,
    pub path: PathBuf,
    pub last_used: String,
    pub java_version: u8,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Installations {
    installations: Vec<Installation>,
}

impl Installations {
    pub fn load() -> Self {
        let path = INSTALLATIONS_DIR.join("installations.json");
        if !path.exists() {
            return Self::default();
        }

        let content = fs::read_to_string(path)
            .expect("Failed to read installations.json");
        serde_json::from_str(&content)
            .expect("Failed to parse installations.json")
    }

    pub fn save(&self) -> Result<(), BackendError> {
        let path = INSTALLATIONS_DIR.join("installations.json");
        let content = serde_json::to_string_pretty(self)
            .expect("Failed to serialize installations");
        fs::write(path, content)
            .map_err(|_| BackendError::FailedToSaveInstallations)?;
        Ok(())
    }

    pub fn add(&mut self, version: String, path: PathBuf, java_version: u8) -> Result<(), BackendError> {
        let installation = Installation {
            version,
            path,
            last_used: chrono::Local::now().to_rfc3339(),
            java_version,
        };
        self.installations.push(installation);
        self.save()
    }

    pub fn get(&self, version: &str) -> Option<&Installation> {
        self.installations.iter().find(|i| i.version == version)
    }

    pub fn update_last_used(&mut self, version: &str) -> Result<(), BackendError> {
        if let Some(installation) = self.installations.iter_mut().find(|i| i.version == version) {
            installation.last_used = chrono::Local::now().to_rfc3339();
            self.save()?;
        }
        Ok(())
    }
}