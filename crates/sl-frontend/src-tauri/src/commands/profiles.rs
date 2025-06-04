use sl_core::profiles::player::{PlayerProfile, PlayerProfiles};

#[tauri::command]
pub fn get_profiles() -> Result<PlayerProfiles, String> {
    PlayerProfiles::load().map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn get_current_profile() -> Result<PlayerProfile, String> {
    let profiles = get_profiles()?;
    let profile = profiles.current_profile();
    match profile {
        Some(profile) => Ok(profile.to_owned()),
        None => Err("Current profile doesn't exist!".to_string())
    }
}

