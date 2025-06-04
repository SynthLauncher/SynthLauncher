use sl_core::instance::{Instance, InstanceType};
use sl_core::instances::Instances;
use sl_utils::utils::errors::BackendError;

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
    create_instance_inner(name, version).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_instance(name: &str) -> Result<(), String> {
    Instances::remove(name).map_err(|e| e.to_string())
}