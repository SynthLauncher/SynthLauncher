use sl_core::VERSION_MANIFEST;
use serde::{Deserialize, Serialize};
use tauri::command;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct MinecraftLoginRequest {
    username: String,
    password: String,
    twofa: Option<String>,
}

#[derive(Serialize)]
pub struct MinecraftLoginResponse {
    accessToken: Option<String>,
    clientToken: Option<String>,
    availableProfiles: Option<serde_json::Value>,
    selectedProfile: Option<serde_json::Value>,
    user: Option<serde_json::Value>,
    error: Option<String>,
    errorMessage: Option<String>,
}

#[tauri::command]
pub async fn get_minecraft_versions() -> Result<Vec<String>, String> {
    Ok(VERSION_MANIFEST.versions().map(|version| version.id.to_owned()).collect())
}

#[command]
pub async fn minecraft_login(
    username: String,
    password: String,
    twofa: Option<String>,
) -> Result<MinecraftLoginResponse, String> {
    let client_token = "test-client-token".to_string();
    let password_final = if let Some(twofa_code) = twofa {
        if !twofa_code.trim().is_empty() {
            format!("{}:{}", password, twofa_code.trim())
        } else {
            password.clone()
        }
    } else {
        password.clone()
    };

    let body = serde_json::json!({
        "username": username,
        "password": password_final,
        "clientToken": client_token,
        "requestUser": true
    });

    println!("Ely.by login payload: {}", body);

    let resp = reqwest::Client::new()
        .post("https://authserver.ely.by/auth/authenticate")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = resp.status();
    let text = resp.text().await.map_err(|e| e.to_string())?;
    println!("Ely.by login response status: {}", status);
    println!("Ely.by login response body: {}", text);
    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({"error": "Invalid JSON", "errorMessage": text}));

    if status == 200 {
        Ok(MinecraftLoginResponse {
            accessToken: json.get("accessToken").and_then(|v| v.as_str()).map(|s| s.to_string()),
            clientToken: json.get("clientToken").and_then(|v| v.as_str()).map(|s| s.to_string()),
            availableProfiles: json.get("availableProfiles").cloned(),
            selectedProfile: json.get("selectedProfile").cloned(),
            user: json.get("user").cloned(),
            error: None,
            errorMessage: None,
        })
    } else if json.get("errorMessage").and_then(|v| v.as_str()) == Some("Account protected with two factor auth.") {
        Ok(MinecraftLoginResponse {
            accessToken: None,
            clientToken: None,
            availableProfiles: None,
            selectedProfile: None,
            user: None,
            error: Some("2fa".to_string()),
            errorMessage: Some("Two-factor authentication required.".to_string()),
        })
    } else {
        Ok(MinecraftLoginResponse {
            accessToken: None,
            clientToken: None,
            availableProfiles: None,
            selectedProfile: None,
            user: None,
            error: json.get("error").and_then(|v| v.as_str()).map(|s| s.to_string()),
            errorMessage: json.get("errorMessage").and_then(|v| v.as_str()).map(|s| s.to_string()),
        })
    }
}
