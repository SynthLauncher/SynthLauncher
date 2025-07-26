use sl_core::launcher::player_accounts::{add_account, remove_account, set_current_account, PlayerAccounts};
use sl_player::PlayerData;

#[tauri::command]
pub fn accounts_get() -> Result<PlayerAccounts , String> {
    PlayerAccounts::load().map_err(|err| err.to_string())
}

#[tauri::command]
pub fn accounts_remove(name: &str) -> Result<(), String> {
    remove_account(name).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn accounts_set_current(name: String) -> Result<(), String> {
    set_current_account(name).map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn accounts_create_offline(name: String) -> Result<(), String> {
    add_account(name, PlayerData::default()).map_err(|e| e.to_string())?;
    Ok(())
}


