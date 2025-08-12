use std::path::Path;

use sl_core::environment::LauncherEnv;
use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;
use tokio::sync::Mutex;
#[tauri::command]
pub fn open_folder(app_handle: AppHandle, folder_path: &Path) -> Result<(), String> {
    app_handle
        .opener()
        .reveal_item_in_dir(&folder_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_synthlauncher_root_folder(
    app_handle: AppHandle,
    launcher_env: State<'_, Mutex<LauncherEnv>>,
) -> Result<(), String> {
    open_folder(app_handle, &launcher_env.lock().await.root())?;
    Ok(())
}
