use tauri::command;
use sl_skin::{ElyBySkinProvider, ElyByTextures};

#[command]
pub async fn get_skin_url(nickname: String) -> Result<Option<String>, String> {
    let provider = ElyBySkinProvider::new();
    provider.fetch_skin_url(&nickname).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_cape_url(nickname: String) -> Result<Option<String>, String> {
    let provider = ElyBySkinProvider::new();
    provider.fetch_cape_url(&nickname).await.map_err(|e| e.to_string())
}

#[command]
pub async fn get_textures(nickname: String) -> Result<Option<ElyByTextures>, String> {
    let provider = ElyBySkinProvider::new();
    provider.fetch_textures(&nickname).await.map_err(|e| e.to_string())
} 