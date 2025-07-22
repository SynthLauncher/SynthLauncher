use std::io::{BufRead, BufReader};

use sl_core::launcher::instances::{
    self,
    game::{get_game_info, GameInfo},
    instance_metadata::{InstanceMetadata, ModLoader},
};
use sl_utils::{elog, errors::BackendError};
use tauri::{AppHandle, Emitter};

use crate::RUNNING_INSTANCES;

#[tauri::command]
pub async fn get_instances() -> Result<Vec<InstanceMetadata>, String> {
    instances::get_all_instances().map_err(|e| e.to_string())
}

async fn create_instance_inner(
    name: String,
    version: String,
    mod_loader: ModLoader,
) -> Result<(), BackendError> {
    InstanceMetadata::create(name, &version, mod_loader, None, None).await?;
    Ok(())
}

#[tauri::command]
pub async fn create_instance(
    name: String,
    version: String,
    mod_loader: ModLoader,
) -> Result<(), String> {
    create_instance_inner(name, version, mod_loader)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_instance(name: &str) -> Result<(), String> {
    instances::remove(name).map_err(|e| e.to_string())
}

async fn launch_instance_inner(name: &str, app_handle: AppHandle) -> Result<(), BackendError> {
    let (instance, _) = instances::get_existing(name)?;
    let loaded_instance = instance.load_init().await?;
    let (child, reader) = loaded_instance.execute().await?;
    let mut reader = BufReader::new(reader);

    RUNNING_INSTANCES.add(name.to_string(), child).await;
    let mut line = String::new();

    app_handle
        .emit("stdout", "Starting instance...")
        .expect("failed to emit the initial data to instance's Console");

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            continue;
        }

        if let Err(e) = app_handle.emit("stdout", &line) {
            elog!("Error emitting stdio to frontend: {}", e);
        }

        line.clear();
    }

    Ok(())
}

#[tauri::command]
pub async fn launch_instance(name: &str, app_handle: AppHandle) -> Result<(), String> {
    launch_instance_inner(name, app_handle)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn kill_instance(name: &str) -> Result<(), String> {
    RUNNING_INSTANCES.kill(name).await;

    Ok(())
}

#[tauri::command]
pub fn load_game_info(name: &str) -> Result<GameInfo, String> {
    get_game_info(name).map_err(|e| e.to_string())
}
