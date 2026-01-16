use serde::{Deserialize, Serialize};
use sl_utils::{errors::BackendError, requester::Requester};

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
    pub refresh_token: String,
    pub username: String,
    pub needs_refresh: bool,
    pub account_type: PlayerAccountType,
}

impl PlayerAccount {
    async fn get_refresh_data(
        &self,
        requester: &Requester,
    ) -> Result<Option<AccountRefreshData>, BackendError> {
        match self.account_type {
            PlayerAccountType::Microsoft => Ok(Some(
                ms_auth::login_refresh(&self.refresh_token, &requester).await?,
            )),
            PlayerAccountType::Offline => Ok(None),
        }
    }

    pub async fn refresh_account_token(
        &mut self,
        requester: &Requester,
    ) -> Result<(), BackendError> {
        if let Some(refresh_data) = self.get_refresh_data(requester).await? {
            self.access_token = Some(refresh_data.access_token);
            self.refresh_token = refresh_data.refresh_token;
        }
        Ok(())
    }
}
