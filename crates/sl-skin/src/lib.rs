use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use anyhow::Result;

const BASE_URL: &str = "http://skinsystem.ely.by";

#[derive(Debug, Deserialize, Serialize)]
pub struct ElyByTextures {
    #[serde(rename = "SKIN")]
    pub skin: Option<ElyByTexture>,
    #[serde(rename = "CAPE")]
    pub cape: Option<ElyByTexture>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ElyByTexture {
    pub url: String,
    pub metadata: Option<ElyByTextureMetadata>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ElyByTextureMetadata {
    pub model: Option<String>,
}

/// Provider for Ely.by skin system
pub struct ElyBySkinProvider {
    client: Client,
}

impl ElyBySkinProvider {
    /// Create a new ElyBySkinProvider with a shared HTTP client
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    /// Fetch the skin URL for a given nickname
    pub async fn fetch_skin_url(&self, nickname: &str) -> Result<Option<String>> {
        let url = format!("http://skinsystem.ely.by/skins/{}.png", nickname);
        let resp = self.client.head(&url).send().await?;
        match resp.status() {
            StatusCode::OK => Ok(Some(url)),
            StatusCode::NOT_FOUND | StatusCode::NO_CONTENT => Ok(None),
            _ => Ok(None),
        }
    }

    /// Fetch the cape URL for a given nickname
    pub async fn fetch_cape_url(&self, nickname: &str) -> Result<Option<String>> {
        let url = format!("http://skinsystem.ely.by/cloaks/{}.png", nickname);
        let resp = self.client.head(&url).send().await?;
        match resp.status() {
            StatusCode::OK => Ok(Some(url)),
            StatusCode::NOT_FOUND | StatusCode::NO_CONTENT => Ok(None),
            _ => Ok(None),
        }
    }

    /// Fetch the textures (skin/cape info) for a given nickname
    pub async fn fetch_textures(&self, nickname: &str) -> Result<Option<ElyByTextures>> {
        let url = format!("{}/textures/{}", BASE_URL, nickname);
        let resp = self.client.get(&url).send().await?;
        match resp.status() {
            StatusCode::OK => Ok(Some(resp.json::<ElyByTextures>().await?)),
            StatusCode::NO_CONTENT | StatusCode::NOT_FOUND => Ok(None),
            _ => Ok(None),
        }
    }
} 