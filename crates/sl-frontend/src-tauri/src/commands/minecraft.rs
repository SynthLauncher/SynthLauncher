use sl_core::VERSION_MANIFEST;


#[tauri::command]
pub async fn get_minecraft_versions() -> Result<Vec<String>, String> {
    Ok(VERSION_MANIFEST.versions().map(|version| version.id.to_owned()).collect())
}
