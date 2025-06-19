use sl_core::launcher::player::{player_profile::PlayerProfile, player_profiles::PlayerProfiles};



#[tauri::command]
pub fn get_profiles() -> Result<PlayerProfiles, String> {
    PlayerProfiles::load().map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn get_current_profile() -> Result<PlayerProfile, String> {
    let profiles = get_profiles()?;
    let profile = profiles.current_profile();
    Ok(profile.into_owned())
}

#[tauri::command]
pub async fn get_other_profiles() -> Result<Vec<PlayerProfile>, String> {
    let profiles = get_profiles()?;
    Ok(profiles.load_other_profiles())
}