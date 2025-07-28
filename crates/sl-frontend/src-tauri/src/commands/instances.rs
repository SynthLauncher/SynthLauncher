use std::path::Path;

use sl_core::launcher::instances::{
    self,
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
    icon: Option<String>,
) -> Result<(), String> {
    InstanceMetadata::create(name, &version, mod_loader, None, icon)
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
pub async fn kill_instance(name: &str, app_handle: AppHandle) -> Result<(), String> {
    RUNNING_INSTANCES.remove(name, &app_handle).await;
    Ok(())
}

#[tauri::command]
pub async fn get_running_instances() -> Vec<String> {
    RUNNING_INSTANCES.list().await
}

#[tauri::command]
pub async fn export_instance(instance_name: &str, output: &Path) -> Result<(), String> {
    let (instance, _) = instances::get_existing(&instance_name).map_err(|e| e.to_string())?;
    let exporter = instance.exporter_to_path(&output.join(instance_name)).map_err(|e| e.to_string())?;
    exporter.export().map_err(|e| e.to_string())?;

    Ok(())
}
