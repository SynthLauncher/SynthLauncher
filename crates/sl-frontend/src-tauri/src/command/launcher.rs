use std::{path::Path, sync::Arc};

use sl_core::environment::LauncherEnv;
use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;
use tokio::sync::RwLock;

#[tauri::command]
pub fn open_folder(app_handle: AppHandle, folder_path: &Path) -> Result<(), String> {
    app_handle
        .opener()
        .reveal_item_in_dir(&folder_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_synthlauncher_root_folder(app_handle: AppHandle, shared_env: State<'_, Arc<RwLock<LauncherEnv>>>) -> Result<(), String> {
    let guard = shared_env.read().await;
    open_folder(app_handle, &guard.root())?;
    Ok(())
}