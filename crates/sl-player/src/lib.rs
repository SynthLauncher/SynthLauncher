use serde::{Deserialize, Serialize};
use sl_utils::{errors::BackendError, requester::Requester};

use uuid::Uuid;
use crate::auth::{AccountRefreshData, ms_auth};

pub mod api;
pub mod auth;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PlayerAccountType {
    Microsoft,
    Offline,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
/// Represents a struct containing account's data
pub struct PlayerAccount {
    pub access_token: Option<String>,
    // Important note: I've noticed a bug, where if the UUID is not valid the world
    // saving doesn't work properly, additionally entering servers too, even if the
    // access token is right
    pub uuid: String,
    pub refresh_token: Option<String>,
    pub username: String,
    pub account_type: PlayerAccountType,
}

fn generate_offline_uuid(name: &str) -> String
{
    let input = format!("OfflinePlayer:{}", name.to_lowercase());
    let mut hash = md5::compute(input.as_bytes()).0;

    // Doing some weird stuff with hashes
    hash[6] = (hash[6] & 0x0f) | 0x30;
    hash[8] = (hash[8] & 0x3f) | 0x80;
    Uuid::from_bytes(hash).to_string()
}

impl PlayerAccount {
    pub fn new(access_token: Option<String>, uuid: String, refresh_token: Option<String>, username: String, account_type: PlayerAccountType) -> Self {
        Self {
            access_token,
            uuid,
            refresh_token,
            username,
            account_type
        }
    }

    pub fn offline_account(username: String) -> Self
    {
        let uuid = generate_offline_uuid(&username);
        Self {
            access_token: None,
            uuid,
            refresh_token: None,
            username: username,
            account_type: PlayerAccountType::Offline
        }
    }
    
    async fn get_refresh_data(
        &self,
        requester: &Requester,
    ) -> Result<Option<AccountRefreshData>, BackendError> {
        match self.account_type {
            PlayerAccountType::Microsoft => {
                if let Some(refresh_token) = &self.refresh_token 
                {
                    Ok(Some(
                        ms_auth::login_refresh(refresh_token, &requester).await?,
                    ))
                } else {
                    Ok(None)
                }         
            },
            PlayerAccountType::Offline => Ok(None),
        }
    }

    pub async fn refresh_account_token(
        &mut self,
        requester: &Requester,
    ) -> Result<(), BackendError> {
        if let Some(refresh_data) = self.get_refresh_data(requester).await? {
            self.access_token = Some(refresh_data.access_token);
            self.refresh_token = Some(refresh_data.refresh_token);
        }
        Ok(())
    }
}
