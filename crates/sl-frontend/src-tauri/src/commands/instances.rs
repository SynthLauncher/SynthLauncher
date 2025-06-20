use std::path::Path;

use sl_core::{launcher::{
    instance::{Instance, InstanceType},
    instances::Instances,
}, HTTP_CLIENT};
use sl_utils::{
    elog,
    utils::{download::download_file, errors::BackendError},
};
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub async fn get_instances() -> Result<Instances, String> {
    Instances::load().map_err(|e| e.to_string())
}

async fn create_instance_inner(name: String, version: String) -> Result<(), BackendError> {
    let mut instance = Instance::new(&name, &version, InstanceType::Vanilla, None)?;
    Instances::add(&instance)?;
    instance.install().await?;

    Ok(())
}

#[tauri::command]
pub async fn create_instance(name: String, version: String) -> Result<(), String> {
    create_instance_inner(name, version)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_progress(app: AppHandle) -> Result<(), String> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<f32>(500);

    let app_handle = app.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            if let Err(e) = app_handle.emit("download-progress", progress) {
                elog!("Failed to emit progress: {e}");
            }
        }
    });

    let path = Path::new("../../file");
    
    download_file(&HTTP_CLIENT, "https://freetestdata.com/wp-content/uploads/2021/09/Free_Test_Data_10MB_OGG.ogg", &path, 3, std::time::Duration::from_secs(5), Some(tx.clone())).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn remove_instance(name: &str) -> Result<(), String> {
    Instances::remove(name).map_err(|e| e.to_string())
}
