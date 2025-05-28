use sl_core::config::config::Config;
use sl_core::instance::{Instance, InstanceType};
use sl_core::instances::Instances;
use sl_utils::utils::errors::BackendError;

async fn get_global_config() -> Result<Config, std::io::Error> {
    Ok(Config::read_global()?)
}

#[tauri::command]
pub async fn get_username() -> Result<String, String> {
    let config = get_global_config().await.map_err(|e| e.to_string())?;
    let username = config
        .get("auth_player_name")
        .ok_or("Missing 'auth_player_name' in config")?
        .to_string();
    Ok(username)
}

#[tauri::command]
pub async fn edit_username(username: &str) -> Result<(), String> {
    let mut config = get_global_config().await.map_err(|e| e.to_string())?;
    config
        .update_config_field("auth_player_name", username)
        .unwrap();

    Ok(())
}

#[tauri::command]
pub async fn get_installations() -> Result<Instances, String> {
    let installations = Instances::load().map_err(|e| e.to_string())?;

    Ok(installations)
}

async fn create_installation_inner(name: String, version: String) -> Result<(), BackendError> {
    let mut instance = Instance::new(&name, &version, InstanceType::Vanilla, None)?;
    Instances::add(&instance)?;
    instance.install().await?;

    Ok(())
}

#[tauri::command]
pub async fn create_installation(name: String, version: String) -> Result<(), String> {
    let results = tokio::task::spawn_local(create_installation_inner(name, version));
    results.await.unwrap().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_installation(name: &str) -> Result<(), String> {
    Instances::remove(name).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn load_all_installations() -> Result<(), String> {
    for instance in Instances::load_all_instances().unwrap().0 {
        Instances::add(&instance).unwrap();
    }
    Ok(())
}

#[tauri::command]
pub async fn launch(name: &str) -> Result<(), String> {
    let instance = Instances::find(name).map_err(|e| e.to_string())?;
    instance.execute(None).await.map_err(|e| e.to_string())?;

    Ok(())
}
