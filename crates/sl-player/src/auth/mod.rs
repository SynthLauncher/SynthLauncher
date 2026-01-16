use serde::Deserialize;

pub mod ms_auth;

#[derive(Debug, Deserialize)]
pub struct AccountRefreshData
{
    pub access_token: String,
    pub refresh_token: String,
}