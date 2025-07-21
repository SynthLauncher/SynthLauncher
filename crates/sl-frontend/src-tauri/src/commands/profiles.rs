use sl_core::launcher::player_profiles::PlayerProfiles;
use sl_player::profile::PlayerProfile;

#[tauri::command]
pub fn get_profiles() -> Result<PlayerProfiles, String> {
    PlayerProfiles::load().map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set_current_profile(index: usize) -> Result<(), String> {
    let mut prfls = PlayerProfiles::load().map_err(|err| err.to_string())?;
    prfls.set_current_profile(index).map_err(|err| err.to_string())?;

    Ok(())
}


#[tauri::command]
pub fn create_offline_profile(name: &str) -> Result<(), String> {
    let mut prfls = PlayerProfiles::load().map_err(|err| err.to_string())?;
    let prfl = PlayerProfile::offline_account(name).map_err(|err| err.to_string())?;
    prfls.add(prfl).map_err(|err| err.to_string())?;
    Ok(())
}