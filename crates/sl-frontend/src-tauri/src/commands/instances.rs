use sl_core::launcher::instances::{
    self,
    game::{get_game_info, GameInfo},
    instance_metadata::{InstanceMetadata, ModLoader},
};
use tauri::AppHandle;

use crate::core::{instances::launch_instance_inner, running_instances::RUNNING_INSTANCES};

#[tauri::command]
pub async fn get_instances() -> Result<Vec<InstanceMetadata>, String> {
    instances::get_all_instances().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_instance(
    name: String,
    version: String,
    mod_loader: ModLoader,
) -> Result<(), String> {
    InstanceMetadata::create(name, &version, mod_loader, None, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn remove_instance(name: &str) -> Result<(), String> {
    instances::remove(name).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn launch_instance(name: &str, app_handle: AppHandle) -> Result<(), String> {
    launch_instance_inner(name, app_handle)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn kill_instance(name: &str) -> Result<(), String> {
    RUNNING_INSTANCES.remove(name).await;
    Ok(())
}

#[tauri::command]
pub fn load_game_info(name: &str, loader: ModLoader) -> Result<GameInfo, String> {
    get_game_info(name, &loader).map_err(|e| e.to_string())
}
