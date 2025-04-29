// Taken from https://github.com/minecraft-rs/auth/

// use std::collections::HashMap;

// use reqwest::Client;
// use serde::{Deserialize, Serialize};


// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct AuthCodeResponse {
//     pub user_code: String,
//     pub device_code: String,
//     pub verification_uri: String,
//     pub expires_in: i64,
//     pub interval: u64,
//     pub message: String
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct AuthTokenResponse {
//     pub token_type: String,
//     pub scope: String,
//     pub expires_in: i64,
//     pub ext_expires_in: i64,
//     pub access_token: String,
//     pub refresh_token: String
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct XboxLiveAuthResponse {
//     pub issue_instant: String,
//     pub not_after: String,
//     pub token: String,
//     pub display_claims: HashMap<String, Vec<HashMap<String, String>>>
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct MinecraftAuthResponse {
//     pub username: String,
//     pub roles: Vec<String>,
//     pub access_token: String,
//     pub expires_in: u32,
//     pub token_type: String
// }

// #[derive(Debug)]
// pub enum AuthServiceError {
//     InvalidAccessToken,
//     UnknownError,
//     Request(reqwest::Error),
//     Json(serde_json::Error)
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// struct AuthServiceErrorMessage {
//     error: String
// }

// pub struct AuthFlow {
//     auth_code_res: Option<AuthCodeResponse>,
//     auth_token_res: Option<AuthTokenResponse>,
//     xbox_auth_res: Option<XboxLiveAuthResponse>,
//     minecraft_res: Option<MinecraftAuthResponse>,
//     client_id: String,
//     client: Client
// }

// impl AuthFlow {
//     pub fn new(client_id: &str) -> Self {
//         Self {
//             client: Client::new(),
//             auth_code_res: None,
//             auth_token_res: None,
//             xbox_auth_res: None,
//             minecraft_res: None,
//             client_id: client_id.to_string()
//         }
//     }

//     pub async fn request_code(&mut self) -> Result<&AuthCodeResponse, AuthServiceError> {
//         let client_id = &self.client_id;

//         let response = self
//             .client
//             .get("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
//             .query(&[
//                 ("client_id", client_id),
//                 ("scope", &"XboxLive.signin offline_access".to_string())
//             ])
//             .send().await.unwrap();
    
//         let data: AuthCodeResponse = response.json().await.unwrap();
//         self.auth_code_res = Some(data);
//         Ok(self.auth_code_res.as_ref()).unwrap()
//     }


// }