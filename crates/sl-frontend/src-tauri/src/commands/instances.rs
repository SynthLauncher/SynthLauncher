use sl_core::launcher::instances::{
    self,
    game::{get_game_info, GameInfo},
    metadata::{InstanceMetadata, ModLoader},
};
use sl_utils::errors::BackendError;

#[tauri::command]
pub async fn get_instances() -> Result<Vec<InstanceMetadata>, String> {
    instances::get_all_instances().map_err(|e| e.to_string())
}

async fn create_instance_inner(
    name: String,
    version: String,
    mod_loader: ModLoader,
) -> Result<(), BackendError> {
    InstanceMetadata::create(&name, &version, mod_loader, None, None).await?;
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

async fn launch_instance_inner(name: &str) -> Result<(), BackendError> {
    let (instance, _) = instances::get_existing(name)?;
    let loaded_instance = instance.load_init().await?;
    loaded_instance.execute().await?;

    Ok(())
}
#[tauri::command]
pub async fn launch_instance(name: &str) -> Result<(), String> {
    launch_instance_inner(name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_game_info(name: &str) -> Result<GameInfo, String> {
    get_game_info(name).map_err(|e| e.to_string())
}

// #[tauri::command]
// pub async fn test_progress(app: AppHandle) -> Result<(), String> {
//     let (tx, mut rx) = tokio::sync::mpsc::channel::<f32>(500);

//     let app_handle = app.clone();
//     tokio::spawn(async move {
//         while let Some(progress) = rx.recv().await {
//             if let Err(e) = app_handle.emit("download-progress", progress) {
//                 elog!("Failed to emit progress: {e}");
//             }
//         }
//     });

//     let path = Path::new("../../file");

//     download_file(
//         &HTTP_CLIENT,
//         "https://freetestdata.com/wp-content/uploads/2021/09/Free_Test_Data_10MB_OGG.ogg",
//         &path,
//         3,
//         std::time::Duration::from_secs(5),
//         Some(tx.clone()),
//     )
//     .await
//     .map_err(|e| e.to_string())?;

//     Ok(())
// }
