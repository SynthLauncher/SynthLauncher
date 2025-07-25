use serde::{Deserialize, Serialize};

pub mod api;
pub mod profile;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerData {
    pub id: String,
    pub access_token: String,
}

impl PlayerData {
    pub fn default() -> Self {
        Self {
            id: "8667ba71-b85a-4004-af54-457a9734eed7".to_string(),
            access_token: "0".to_string(),
        }
    }
}
