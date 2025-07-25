use sl_core::launcher::player_accounts::PlayerAccounts;
use sl_player::PlayerData;


#[tauri::command]
pub fn get_accounts() -> Result<PlayerAccounts , String> {
    PlayerAccounts::load().map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set_current_account(name: String) -> Result<(), String> {
    let mut accounts = PlayerAccounts::load().map_err(|err| err.to_string())?;
    accounts.set_current(name).map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn create_offline_account(name: String) -> Result<(), String> {
    let mut accounts = PlayerAccounts::load().map_err(|err| err.to_string())?;
    accounts.accounts.insert(name, PlayerData::default());
    Ok(())
}
