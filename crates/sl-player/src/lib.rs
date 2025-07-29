use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

pub mod api;

const NS: Uuid = uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c8");

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerData {
    pub id: String,
    pub access_token: String,
}

impl PlayerData {
    pub fn offline(name: &str) -> Self {
        Self {
            id: Uuid::new_v3(&NS, format!("OfflinePlayer:{name}").as_bytes()).to_string(),
            access_token: "0".to_string(),
        }
    }
}
