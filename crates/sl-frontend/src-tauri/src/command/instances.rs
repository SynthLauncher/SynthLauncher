use serde::{Deserialize, Serialize};
use sl_core::{
    environment::LauncherEnv,
    instances::instance_metadata::{InstanceMetadata, ModLoader},
};
use tauri::State;
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Instance {
    #[serde(flatten)]
    metadata: InstanceMetadata,
    icon: Option<Vec<u8>>,
}

#[tauri::command]
pub async fn create_instance(
    instance_name: String,
    game_version: &str,
    loader: ModLoader,
    loader_version: Option<String>,
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<InstanceMetadata, String> {
    let env = launcher_env.read().await;

    let instance = env.instances()
        .create_instance(instance_name, &game_version, loader, loader_version)
        .await.map_err(|e| e.to_string())?;
    
    Ok(instance)
}

#[tauri::command]
pub async fn get_instance(
    name: &str,
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<Instance, String> {
    let env = launcher_env.read().await;

    let instance_metadata = env
        .instances()
        .get_existing(name)
        .map_err(|e| e.to_string())?
        .0;
    let icon = instance_metadata.get_instance_icon(&env.instances()).await;

    Ok(Instance {
        metadata: instance_metadata,
        icon: icon,
    })
}

#[tauri::command]
pub async fn get_all_instances(
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<Vec<Instance>, String> {
    let mut instances = Vec::new();
    let env = launcher_env.read().await;

    let instance_metadatas = env
        .instances()
        .get_all_instances()
        .await
        .map_err(|e| e.to_string())?;

    for metadata in instance_metadatas {
        let icon = metadata.get_instance_icon(&env.instances()).await;
        instances.push(Instance {
            metadata: metadata,
            icon: icon,
        });
    }

    Ok(instances)
}

#[tauri::command]
pub async fn launch_instance(
    name: &str,
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<(), String> {
    let env = launcher_env.read().await;

    let instance_metadata = env
        .instances()
        .get_existing(name)
        .map_err(|e| e.to_string())?
        .0;

    instance_metadata
        .load_init(&env.instances())
        .await
        .map_err(|e| e.to_string())?
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
