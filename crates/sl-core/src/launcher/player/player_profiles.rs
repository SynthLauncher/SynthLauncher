use std::{borrow::Cow, fs::{self, OpenOptions}};

use serde::{Deserialize, Serialize};
use sl_utils::errors::BackendError;

use crate::{launcher::player::player_profile::PlayerProfile, PROFILES_PATH};

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
        let content = fs::read_to_string(&PROFILES_PATH.as_path())?;
        Ok(serde_json::from_str(&content).unwrap_or(PlayerProfiles::new()))
    }

    pub fn overwrite(profiles: &PlayerProfiles) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(PROFILES_PATH.as_path())?;

        serde_json::to_writer_pretty(file, &profiles)?;

        Ok(())
    }

    pub fn add(&mut self, profile: PlayerProfile) -> std::io::Result<()> {
        if !self
            .profiles
            .iter()
            .any(|existing| existing.data.username == profile.data.username)
        {
            self.profiles.push(profile);
        }

        Self::overwrite(&self)?;

        Ok(())
    }

    pub fn find(
        &self,
        name: &str,
        premium: bool,
    ) -> Result<(Option<&PlayerProfile>, usize), BackendError> {
        if let Some((index, profile)) = self.profiles.iter().enumerate().find(|(_, profile)| {
            profile.data.username.eq_ignore_ascii_case(name) && profile.premium == premium
        }) {
            return Ok((Some(profile), index));
        }

        Ok((None, usize::MAX))
    }

    pub fn current_profile(&self) -> Cow<'_, PlayerProfile> {
        self.profiles
            .get(self.current_profile_index)
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(PlayerProfile::default_profile()))
    }

    pub fn load_other_profiles(&self) -> Vec<PlayerProfile> {
        self.profiles
            .iter()
            .enumerate()
            .filter_map(|(i, profile)| {
                if i != self.current_profile_index {
                    Some(profile.clone()) // clone here
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn set_current_profile(&mut self, index: usize) -> Result<(), BackendError> {
        self.current_profile_index = index;
        Self::overwrite(&self)?;
        Ok(())
    }
}
