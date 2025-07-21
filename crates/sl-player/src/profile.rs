use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderValue},
};
use serde::{Deserialize, Serialize};
use sl_utils::errors::{BackendError, HttpError};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerProfileData {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerProfile {
    pub data: PlayerProfileData,
    pub access_token: String,
    pub premium: bool
}

impl PlayerProfile {
    pub fn default_profile() -> PlayerProfile {
        PlayerProfile {
            access_token: "0".to_string(),
            data: PlayerProfileData {
                name: "synther".to_string(),
                id: "8667ba71-b85a-4004-af54-457a9734eed7".to_string(),
            },
            premium: false
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

        Ok(PlayerProfile { data, access_token, premium: true })
    }

    pub fn offline_account(name: impl Into<String>) -> Result<PlayerProfile, BackendError> {
        Ok(PlayerProfile {
            access_token: "0".to_string(),
            data: PlayerProfileData {
                name: name.into(),
                id: "8667ba71-b85a-4004-af54-457a9734eed7".to_string(),
            },
            premium: false
        })
    }
}
