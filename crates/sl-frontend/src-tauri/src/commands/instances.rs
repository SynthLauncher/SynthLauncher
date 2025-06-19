use sl_core::launcher::{
    instance::{Instance, InstanceType},
    instances::Instances,
};
use sl_utils::{
    elog,
    utils::{download::Progress, errors::BackendError},
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
pub async fn test_progress(app: AppHandle) {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Progress<'static>>(100);

    let app_handle = app.clone();
    tokio::spawn(async move {
        while let Some(progress) = rx.recv().await {
            if let Err(e) = app_handle.emit("download-progress", progress) {
                elog!("Failed to emit progress: {e}");
            }
        }
    });

    for i in 0..=100 {
        // Simulate work
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Send progress update
        if let Err(e) = tx
            .send(Progress {
                name: "s",
                downloaded: i,
                total: Some(100),
            })
            .await
        {
            eprintln!("Failed to send progress: {}", e);
        }
    }
}

#[tauri::command]
pub async fn remove_instance(name: &str) -> Result<(), String> {
    Instances::remove(name).map_err(|e| e.to_string())
}
