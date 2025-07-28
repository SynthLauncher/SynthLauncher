use sl_core::{INSTANCES_DIR, LAUNCHER_DIR};
use std::path::Path;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn open_folder(app_handle: AppHandle, folder_path: &Path) -> Result<(), String> {
    app_handle
        .opener()
        .reveal_item_in_dir(&folder_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_synthlauncher_folder(app_handle: AppHandle) -> Result<(), String> {
    let folder_path = &*LAUNCHER_DIR;
    open_folder(app_handle, &folder_path)?;
    Ok(())
}

#[tauri::command]
pub fn open_instance_folder(app_handle: AppHandle, name: String) -> Result<(), String> {
    let folder_path = &*INSTANCES_DIR.join(name);
    open_folder(app_handle, &folder_path)?;
    Ok(())
}
