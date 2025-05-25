use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Client,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerProfile {
    #[serde(skip)]
    pub access_token: String,
    #[serde(skip)]
    pub premium: bool,
    #[serde(rename = "name")]
    pub username: String,
    #[serde(rename = "id")]
    pub uuid: String,
}

impl PlayerProfile {
    pub async fn new(access_token: String) -> Result<PlayerProfile, Box<dyn std::error::Error>> {
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
        Ok(profile)
    }
}
