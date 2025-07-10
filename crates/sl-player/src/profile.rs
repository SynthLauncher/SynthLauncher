use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderValue},
};
use serde::{Deserialize, Serialize};
use sl_utils::errors::{BackendError, HttpError};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerProfile {
    pub name: String,
    pub id: String,
    #[serde(skip_deserializing)]
    pub access_token: String,
}

impl PlayerProfile {
    pub fn default_profile() -> PlayerProfile {
        PlayerProfile {
            name: "synther".to_string(),
            id: "8667ba71-b85a-4004-af54-457a9734eed7".to_string(),
            access_token: "0".to_string(),
        }
    }

    pub fn offline_account(username: String) -> PlayerProfile {
        PlayerProfile {
            access_token: "0".to_string(),
            name: username,
            id: "8667ba71-b85a-4004-af54-457a9734eed7".to_string(),
        }
    }

    pub async fn premium_account(access_token: String) -> Result<PlayerProfile, BackendError> {
        let client = Client::new();
        let res = client
            .get("https://api.minecraftservices.com/minecraft/profile")
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", access_token))?,
            )
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(BackendError::HttpError(HttpError::Status(res.status())));
        }

        let mut profile: PlayerProfile = res.json().await?;
        profile.access_token = access_token;

        Ok(profile)
    }
}
