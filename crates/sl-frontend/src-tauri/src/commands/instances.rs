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
    let emit_target = format!("{name}-console");

    let (instance, _) = instances::get_existing(name)?;
    let loaded_instance = instance.load_init().await?;

    let (mut child, reader) = loaded_instance.execute().await?;
    let mut reader = BufReader::new(reader);

    RUNNING_INSTANCES.add(name.to_string()).await;

    let mut line = String::new();

    let emit = |line: &str| app_handle.emit(&emit_target, line);
    emit("Starting instance...")
        .expect("failed to emit the initial data to the instance's Console");

    let mut dead_peacfully = false;
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            continue;
        }

        if let Err(e) = emit(&line) {
            elog!("Error emitting stdio to frontend: {}", e);
        }
        line.clear();

        match child.try_wait() {
            Ok(Some(status)) => {
                emit(&format!("EXIT WITH CODE {}\n", status.code().unwrap_or(-1)))
                    .expect("failed to emit end data");
                dead_peacfully = true;
                break;
            }
            Ok(None) => {}
            Err(_) => break,
        }

        if !RUNNING_INSTANCES.is_alive(name).await {
            break;
        }
    }

    // in case it dead peacefully or an error occurred
    RUNNING_INSTANCES.remove(&name).await;

    // in case it was removed from the list or an error occurred
    if !dead_peacfully {
        _ = child.kill().await;
        emit("DEAD ABNORMALLY\n").expect("failed to emit end data");
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
    RUNNING_INSTANCES.remove(name).await;

    Ok(())
}

#[tauri::command]
pub fn load_game_info(name: &str) -> Result<GameInfo, String> {
    get_game_info(name).map_err(|e| e.to_string())
}
