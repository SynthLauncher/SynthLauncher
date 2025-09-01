use reqwest::header::{HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use sl_utils::{errors::{BackendError, HttpError}, requester::Requester};

#[derive(Debug, Deserialize)]
struct QueryPlayerUsernameResponse {
    name: String,
}

pub async fn get_premium_account_name(requester: &Requester, access_token: &str) -> Result<String, BackendError> {
    let res = requester
        .client()
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

    let json: QueryPlayerUsernameResponse = res.json().await?;

    Ok(json.name)
}
