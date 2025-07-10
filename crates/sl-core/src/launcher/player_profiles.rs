use std::fs::OpenOptions;

use serde::{Deserialize, Serialize};
use sl_player::profile::PlayerProfile;
use sl_utils::errors::BackendError;

use crate::PROFILES_PATH;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfiles {
    pub current_profile_index: usize,
    pub profiles: Vec<PlayerProfile>,
}

impl PlayerProfiles {
    pub fn new() -> Self {
        PlayerProfiles {
            current_profile_index: 0,
            profiles: vec![PlayerProfile::default_profile()],
        }
    }

    pub fn load() -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&PROFILES_PATH.as_path())?;
        Ok(serde_json::from_str(&content).unwrap_or(Self::new()))
    }

    pub fn overwrite(new_profiles: &PlayerProfiles) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(PROFILES_PATH.as_path())?;

        serde_json::to_writer_pretty(file, &new_profiles)?;

        Ok(())
    }

    pub fn add(&mut self, profile: PlayerProfile) -> std::io::Result<()> {
        if !self
            .profiles
            .iter()
            .any(|existing| existing.name == profile.name)
        {
            self.profiles.push(profile);
        }

        Self::overwrite(&self)?;

        Ok(())
    }

    pub fn set_current_profile(&mut self, index: usize) -> Result<(), BackendError> {
        self.current_profile_index = index;
        Self::overwrite(&self)?;
        Ok(())
    }
}
