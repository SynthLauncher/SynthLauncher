//! Taken from <https://github.com/Mrmayman/quantumlauncher/blob/main/crates/ql_instances/src/auth/ms.rs>
//! # Minecraft Authentication for Microsoft Accounts
//!
//! This module allows you to log into Minecraft with
//! your paid Microsoft account.
//!
//! Taken from <https://github.com/minecraft-rs/auth>
//!
//! ## Modifications:
//! - Changed to `reqwest::Client` and `async`
//!   from `reqwest::blocking::Client`
//! - Changed error handling code
//! - Split it up into clean, independent functions
//!
//! # Login Process
//! ## 1) Adding a new account
//! If you are logging in and adding a new account, then:
//!
//! ```no_run
//! # async fn do1() -> Result<(), Box<dyn std::error::Error>> {
//! use ql_instances::auth::ms::login_1_link;
//! let auth_code_response = login_1_link().await?;
//! // AuthCodeResponse { verification_uri, user_code, .. }
//! # Ok(()) }
//! ```
//!
//! Now we wait for user to open the `verification_uri` link in browser,
//! login with their account,
//! then enter `user_code`.
//!
//! ```no_run
//! # async fn do2() -> Result<(), Box<dyn std::error::Error>> {
//! # // Default construction
//! # let auth_code_response = ql_instances::auth::ms::AuthCodeResponse {
//! #     user_code: String::new(),
//! #     device_code: String::new(),
//! #     verification_uri: String::new(),
//! #     expires_in: 0,
//! #     interval: 0,
//! #     message: String::new(),
//! # };
//! use ql_instances::auth::ms::login_3_xbox;
//! use ql_instances::auth::ms::login_2_wait;
//!
//! let auth_token_response = login_2_wait(auth_code_response).await?;
//! // AuthTokenResponse { access_token, refresh_token }
//!
//! let account_data = login_3_xbox(auth_token_response, None, true).await?;
//! // AccountData { access_token, uuid, username, refresh_token, needs_refresh }
//! # Ok(()) }
//! ```
//!
//! Now save the `username` and corresponding `refresh_token` to disk
//! and play the game with `access_token`.
//!
//! ## 2) Refreshing the account on every play session
//! After starting the launcher later, to refresh
//! the token, we do
//!
//! ```no_run
//! # async fn do3() -> Result<(), Box<dyn std::error::Error>> {
//! # let username = String::new();
//! # let refresh_token = String::new();
//! use ql_instances::auth::ms::login_refresh;
//! let account_data = login_refresh(username, refresh_token, None).await?;
//! # Ok(()) }
//! ```

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sl_utils::{
    errors::{BackendError, HttpError, MicrosoftAuthServiceError},
    log,
    requester::Requester,
};
use std::collections::HashMap;
use thiserror::Error;

use crate::{auth::AccountRefreshData, PlayerAccount, PlayerAccountType};

const CLIENT_ID: &str = "74909cec-49b6-4fee-aa60-1b2a57ef72e1";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCodeResponse {
    pub user_code: String,
    device_code: String,
    pub verification_uri: String,
    expires_in: i64,
    interval: u64,
    message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthTokenResponse {
    token_type: String,
    scope: String,
    expires_in: i64,
    ext_expires_in: i64,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveAuthResponse {
    issue_instant: String,
    not_after: String,
    token: String,
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct MinecraftAuthResponse {
    pub username: String,
    roles: Vec<String>,
    pub access_token: String,
    expires_in: u32,
    token_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AuthServiceErrorMessage {
    error: String,
}

#[derive(Debug, Deserialize, Clone)]
struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
struct MsaResponseError {
    pub path: String,
    pub error: String,
    pub errorMessage: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
struct MinecraftFinalDetails {
    id: Option<String>,
    name: String,
}

// #[derive(Debug, Error)]
// pub enum Error {
//     #[error("{AUTH_ERR_PREFIX}{0}")]
//     Request(#[from] RequestError),
//     #[error("{AUTH_ERR_PREFIX}{0}")]
//     Json(#[from] JsonError),
//     #[error("{AUTH_ERR_PREFIX}Invalid account access token!")]
//     InvalidAccessToken,
//     #[error("{AUTH_ERR_PREFIX}An unknown error has occurred (code: {0})\n\nThis is a major bug! Please report in discord.")]
//     UnknownError(StatusCode),
//     #[error("{AUTH_ERR_PREFIX}missing JSON field: {0}")]
//     MissingField(String),
//     #[error("{AUTH_ERR_PREFIX}no uuid found for account")]
//     NoUuid,
//     #[error("{AUTH_ERR_PREFIX}{0}")]
//     Response(MsaResponseError),

//     #[error("Your Microsoft account doesn't own Minecraft!\nJust enter the username in the text box instead of logging in.")]
//     DoesntOwnGame,
// }

async fn login_in_xbox_live(
    auth_token: &AuthTokenResponse,
    requester: &Requester,
) -> Result<XboxLiveAuthResponse, BackendError> {
    let xbox_authenticate_json = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": &format!("d={}", auth_token.access_token)
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });
    
    let xbox_res = requester
        .client()
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&xbox_authenticate_json)
        .send()
        .await?
        .text()
        .await?;
    
    let xbox_res: XboxLiveAuthResponse = serde_json::from_str(&xbox_res)?;
    Ok(xbox_res)
}

async fn login_in_minecraft(
    xbox_res: &XboxLiveAuthResponse,
    requester: &Requester
) -> Result<MinecraftAuthResponse, BackendError>
{
    let xbox_token = &xbox_res.token;
    // TODO: Remove unwrap hell
    let user_hash = &xbox_res
        .display_claims
        .get("xui")
        .unwrap()
        .first()
        .unwrap()
        .get("uhs")
        .unwrap();

    let xbox_security_token_res = requester
        .client()
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbox_token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }))
        .send()
        .await?
        .text()
        .await?;

    let xbox_security_token_res: XboxLiveAuthResponse =
        serde_json::from_str(&xbox_security_token_res)?;

    let xbox_security_token = &xbox_security_token_res.token;

    let minecraft_resp = requester
        .client()
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&json!({
            "identityToken":
                format!(
                    "XBL3.0 x={user_hash};{xbox_security_token}"
                )
        }))
        .send()
        .await?
        .text()
        .await?;

    let minecraft_resp: MinecraftAuthResponse =
        serde_json::from_str(&minecraft_resp)?;
    Ok(minecraft_resp)
}

async fn get_final_details(
    minecraft_res: &MinecraftAuthResponse,
    requester: &Requester
) -> Result<MinecraftFinalDetails, BackendError>
{
    let text = requester
        .client()
        .get("https://api.minecraftservices.com/minecraft/profile")
        .header("Accept", "application/json")
        .bearer_auth(&minecraft_res.access_token)
        .send()
        .await?
        .text()
        .await?;

    // Elaborate on the error more!
    let info = serde_json::from_str::<MinecraftFinalDetails>(&text)?;
    Ok(info)
}

async fn check_minecraft_ownership(access_token: &str, requester: &Requester) -> Result<bool, BackendError>
{
    #[derive(Deserialize)]
    struct Ownership {
        items: Vec<serde_json::Value>
    }

    let response = requester
        .client()
                .get("https://api.minecraftservices.com/entitlements/mcstore")
        .bearer_auth(access_token)
        .send()
        .await?
        .text()
        .await?;
    let response: Ownership = serde_json::from_str(&response)?;
    Ok(!response.items.is_empty())
}

pub async fn login_refresh(
    refresh_token: &str,
    requester: &Requester,
) -> Result<AccountRefreshData, BackendError> {
    let respsonse: String = requester
        .client()
        .post("https://login.live.com/oauth20_token.srf")
        .form(&[
            ("client_id", CLIENT_ID),
            ("refresh_token", &refresh_token),
            ("grant_type", "refresh_token"),
            ("redirect_uri", "https://login.live.com/oauth20_desktop.srf"),
            ("scope", "XboxLive.signin offline_access"),
        ])
        .send()
        .await?
        .text()
        .await?;

    let data: RefreshResponse = serde_json::from_str(&respsonse)?;
    let refresh_data = AccountRefreshData { access_token: data.access_token, refresh_token: data.refresh_token };
    Ok(refresh_data)
}

pub async fn login_1_link(requester: &Requester) -> Result<AuthCodeResponse, BackendError> {
    let response = requester
        .client()
        .get("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
        .query(&[
            ("client_id", CLIENT_ID),
            ("scope", "XboxLive.signin offline_access"),
        ])
        .send()
        .await?
        .text()
        .await?;
    let data: AuthCodeResponse = serde_json::from_str(&response)?;
    log!(
        "Go to {} and enter code {}",
        data.verification_uri,
        data.user_code
    );
    Ok(data)
}


pub async fn login_2_wait(response: AuthCodeResponse, requester: &Requester) -> Result<AuthTokenResponse, BackendError> {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(response.interval + 1)).await;

        let code_resp = requester
            .client()
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&[
                ("client_id", CLIENT_ID),
                ("scope", "XboxLive.signin offline_access"),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("device_code", &response.device_code),
            ])
            .send()
            .await?;

        match code_resp.status() {
            StatusCode::BAD_REQUEST => {
                let txt = code_resp.text().await?;
                let error: AuthServiceErrorMessage = serde_json::from_str(&txt)?;
                match &error.error as &str {
                    "authorization_declined" | "expired_token" | "invalid_grant" => {
                        return Err(BackendError::LauncherFailedToInitialize);
                    }
                    _ => {}
                }
            }

            StatusCode::OK => {
                let text = code_resp.text().await?;
                let response: AuthTokenResponse = serde_json::from_str(&text)?;
                return Ok(response);
            }
            code => {
                return Err(BackendError::LauncherFailedToInitialize);
            }
        }
    }
}

pub async fn login_3_xbox(
    auth_token: AuthTokenResponse,
    check_ownership: bool,
    requester: &Requester
) -> Result<PlayerAccount, BackendError>
{
    // let steps = if check_ownership { 5 } else { 4 };
    let xbox = login_in_xbox_live(&auth_token, requester).await?;
    let minecraft = login_in_minecraft(&xbox, requester).await?;
    let final_detailss = get_final_details(&minecraft, requester).await?;

    // if check_ownership {
    //     let owns_game = check_ownership
    // }

    let data = PlayerAccount {
        access_token: Some(minecraft.access_token),
        uuid: final_detailss.id.ok_or(BackendError::HttpError(HttpError::Timeout))?,
        refresh_token: auth_token.refresh_token,
        needs_refresh: false,
        username: final_detailss.name,
        account_type: PlayerAccountType::Microsoft
    };

    Ok(data)
}

struct AuthFlow {
    auth_code_res: Option<AuthCodeResponse>,
    auth_token_res: Option<AuthTokenResponse>,
    xbox_auth_res: Option<XboxLiveAuthResponse>,
    minecraft_res: Option<MinecraftAuthResponse>,
    client_id: String,

    client: Client,
}

impl AuthFlow {
    pub fn new(client_id: &str) -> Self {
        Self {
            client: Client::new(),
            auth_code_res: None,
            auth_token_res: None,
            xbox_auth_res: None,
            minecraft_res: None,
            client_id: client_id.to_string(),
        }
    }

    pub async fn request_code(&mut self) -> Result<&AuthCodeResponse, BackendError> {
        let client_id = &self.client_id;

        let response = self
            .client
            .get("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
            .query(&[
                ("client_id", client_id),
                ("scope", &"XboxLive.signin offline_access".to_string()),
            ])
            .send()
            .await?;

        let bytes = response.bytes().await?;
        let body_str = std::str::from_utf8(&bytes).unwrap();
        log!("Device code response: {}", body_str);

        let data: AuthCodeResponse = serde_json::from_slice(&bytes)?;
        self.auth_code_res = Some(data);
        return Ok(self.auth_code_res.as_ref().unwrap());
    }

    pub async fn wait_for_login(&mut self) -> Result<&AuthTokenResponse, BackendError> {
        let auth_code = self.auth_code_res.as_ref().unwrap();
        let client_id = &self.client_id;

        loop {
            std::thread::sleep(std::time::Duration::from_secs(auth_code.interval + 1));

            let code_resp = self
                .client
                .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
                .form(&[
                    ("client_id", client_id),
                    ("scope", &"XboxLive.signin offline_access".to_string()),
                    (
                        "grant_type",
                        &"urn:ietf:params:oauth:grant-type:device_code".to_string(),
                    ),
                    ("device_code", &auth_code.device_code),
                ])
                .send()
                .await?;

            match code_resp.status() {
                StatusCode::BAD_REQUEST => {
                    let bytes = code_resp.bytes().await?;
                    let error: AuthServiceErrorMessage = serde_json::from_slice(&bytes)?;
                    match &error.error as &str {
                        "authorization_declined" => {
                            return Err(BackendError::MicrosoftAuthServiceError(
                                MicrosoftAuthServiceError::InvalidAccessToken,
                            ));
                        }
                        "expired_token" => {
                            return Err(BackendError::MicrosoftAuthServiceError(
                                MicrosoftAuthServiceError::InvalidAccessToken,
                            ));
                        }
                        "invalid_grant" => {
                            return Err(BackendError::MicrosoftAuthServiceError(
                                MicrosoftAuthServiceError::InvalidAccessToken,
                            ));
                        }
                        _ => {
                            continue;
                        }
                    }
                }

                StatusCode::OK => {
                    let bytes = code_resp.bytes().await?;
                    let response: AuthTokenResponse = serde_json::from_slice(&bytes)?;
                    self.auth_token_res = Some(response);
                    return Ok(self.auth_token_res.as_ref().unwrap());
                }
                _ => {
                    return Err(BackendError::MicrosoftAuthServiceError(
                        MicrosoftAuthServiceError::UnknownError,
                    ));
                }
            }
        }
    }

    pub async fn login_in_xbox_live(&mut self) -> Result<&XboxLiveAuthResponse, BackendError> {
        let auth_token = self.auth_token_res.as_ref().unwrap();

        let xbox_authenticate_json = json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": &format!("d={}", auth_token.access_token)
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        });

        let xbox_res: XboxLiveAuthResponse = self
            .client
            .post("https://user.auth.xboxlive.com/user/authenticate")
            .json(&xbox_authenticate_json)
            .send()
            .await?
            .json()
            .await?;

        self.xbox_auth_res = Some(xbox_res);
        return Ok(self.xbox_auth_res.as_ref().unwrap());
    }

    pub async fn login_in_minecraft(&mut self) -> Result<&MinecraftAuthResponse, BackendError> {
        let xbox_res = self.xbox_auth_res.as_ref().unwrap();
        let xbox_token = &xbox_res.token;
        let user_hash = &xbox_res.display_claims["xui"][0]["uhs"];

        let xbox_security_token_res: XboxLiveAuthResponse = self
            .client
            .post("https://xsts.auth.xboxlive.com/xsts/authorize")
            .json(&json!({
                "Properties": {
                    "SandboxId": "RETAIL",
                    "UserTokens": [xbox_token]
                },
                "RelyingParty": "rp://api.minecraftservices.com/",
                "TokenType": "JWT"
            }))
            .send()
            .await?
            .json()
            .await?;

        let xbox_security_token = &xbox_security_token_res.token;

        let minecraft_resp: MinecraftAuthResponse = self
            .client
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .json(&json!({
                "identityToken":
                    format!(
                        "XBL3.0 x={user_hash};{xsts_token}",
                        user_hash = user_hash,
                        xsts_token = xbox_security_token
                    )
            }))
            .send()
            .await?
            .json()
            .await?;

        self.minecraft_res = Some(minecraft_resp);
        return Ok(self.minecraft_res.as_ref().unwrap());
    }
}
