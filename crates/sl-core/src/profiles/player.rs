use std::fs::{self, OpenOptions};

use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Client,
};
use serde::{Deserialize, Serialize};
use sl_utils::utils::errors::BackendError;

use crate::PROFILES_PATH;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfile {
    #[serde(rename = "name")]
    pub username: String,
    #[serde(rename = "id")]
    pub uuid: String,
    #[serde(skip)]
    pub access_token: String,
    #[serde(skip)]
    pub premium: bool,
}

impl PlayerProfile {
    pub async fn premium_account(
        access_token: String,
    ) -> Result<PlayerProfile, Box<dyn std::error::Error>> {
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
            return Err(format!("API error: {}", response.status()).into());
        }

        let mut profile: PlayerProfile = response.json().await?;
        profile.access_token = access_token;
        profile.premium = true;
        Ok(profile)
    }

    pub async fn offline_account(
        username: String,
    ) -> Result<PlayerProfile, Box<dyn std::error::Error>> {
        Ok(PlayerProfile {
            access_token: "0".to_string(),
            premium: false,
            username,
            uuid: "0".to_string(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfiles(Vec<PlayerProfile>);

impl PlayerProfiles {
    pub fn load() -> std::io::Result<Self> {
        let content = fs::read_to_string(&PROFILES_PATH.as_path())?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn overwrite(profiles: &PlayerProfiles) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(PROFILES_PATH.as_path())?;

        serde_json::to_writer_pretty(file, &profiles)?;

        Ok(())
    }

    pub fn add(profile: PlayerProfile) -> std::io::Result<()> {
        let mut existing_profiles = Self::load()?;

        if !existing_profiles
            .0
            .iter()
            .any(|existing| existing.username == profile.username)
        {
            existing_profiles.0.push(profile);
        }

        Self::overwrite(&existing_profiles)?;

        Ok(())
    }

    pub fn find(name: &str) -> Result<Option<PlayerProfile>, BackendError> {
        let profiles = Self::load()?;

        if let Some(profile) = profiles
            .0
            .into_iter()
            .find(|profile| profile.username == name)
        {
            return Ok(Some(profile));
        }

        Ok(None)
    }
}
