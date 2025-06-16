use std::fs::{self, OpenOptions};

use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Client,
};
use serde::{Deserialize, Serialize};
use sl_utils::utils::errors::{BackendError, HttpError};

use crate::PROFILES_PATH;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerProfileData {
    #[serde(rename = "name")]
    pub username: String,
    #[serde(rename = "id")]
    pub uuid: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerProfile {
    pub data: PlayerProfileData,
    pub access_token: String,
    pub premium: bool,
}

impl PlayerProfile {
    pub fn default_profile() -> PlayerProfile {
        PlayerProfile {
            access_token: "0".to_string(),
            premium: false,
            data: PlayerProfileData {
                username: "synther".to_string(),
                uuid: "8667ba71-b85a-4004-af54-457a9734eed7".to_string(),
            },
        }
    }

    pub async fn premium_account(access_token: String) -> Result<PlayerProfile, BackendError> {
        let client = Client::new();
        let response = client
            .get("https://api.minecraftservices.com/minecraft/profile")
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", access_token))?,
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BackendError::HttpError(HttpError::Status(
                response.status(),
            )));
        }

        let data: PlayerProfileData = response.json().await?;

        Ok(PlayerProfile {
            data,
            access_token,
            premium: true,
        })
    }

    pub async fn offline_account<U: Into<String>>(
        username: U,
    ) -> Result<PlayerProfile, BackendError> {
        let username = username.into();
        Ok(PlayerProfile {
            access_token: "0".to_string(),
            premium: false,
            data: PlayerProfileData {
                username,
                uuid: "8667ba71-b85a-4004-af54-457a9734eed7".to_string(),
            },
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfiles {
    current_profile_index: usize,
    profiles: Vec<PlayerProfile>,
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

    pub fn current_profile(&self) -> Option<&PlayerProfile> {
        self.profiles.get(self.current_profile_index)
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
