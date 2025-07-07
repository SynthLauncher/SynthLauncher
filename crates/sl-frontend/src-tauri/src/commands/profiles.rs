use sl_core::launcher::player::{player_profile, player_profile::PlayerProfile, player_profiles::PlayerProfiles};
use tauri::command;

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

#[command]
pub async fn set_elyby_profile(username: String, uuid: String, access_token: String) -> Result<(), String> {
    let mut profiles = PlayerProfiles::load().map_err(|e| e.to_string())?;
    let new_profile = PlayerProfile {
        data: player_profile::PlayerProfileData {
            username: username.clone(),
            uuid: uuid.clone(),
        },
        access_token,
        premium: false, // Ely.by is not premium
    };
    // Check if profile exists, update or add
    let (existing, idx) = profiles.find(&username, false).unwrap_or((None, usize::MAX));
    if existing.is_some() {
        profiles.profiles[idx] = new_profile;
        profiles.set_current_profile(idx).map_err(|e| e.to_string())?;
    } else {
        profiles.profiles.push(new_profile);
        profiles.set_current_profile(profiles.profiles.len() - 1).map_err(|e| e.to_string())?;
    }
    PlayerProfiles::overwrite(&profiles).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn reset_profile_to_default() -> Result<(), String> {
    let mut profiles = PlayerProfiles::load().map_err(|e| e.to_string())?;
    let default_profile = PlayerProfile::default_profile();
    // Replace all profiles with just the default
    profiles.profiles = vec![default_profile];
    profiles.current_profile_index = 0;
    PlayerProfiles::overwrite(&profiles).map_err(|e| e.to_string())?;
    Ok(())
}