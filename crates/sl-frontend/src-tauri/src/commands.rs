use sl_core::config::config::Config;
use sl_core::installations::{Installation, Installations};

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
pub async fn get_installations() -> Result<Installations, String> {
    let installations = Installations::load().map_err(|e| e.to_string())?;

    Ok(installations)
}

#[tauri::command]
pub async fn create_installation(name: &str, version: &str) -> Result<(), String> {
    let mut instance = Installation::new(&name, &version).unwrap();
    Installations::add(&instance).unwrap();
    instance.install().await.unwrap();

    Ok(())
}

#[tauri::command]
pub async fn remove_installation(name: &str) -> Result<(), String> {
    Installations::remove(name).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn load_all_installations() -> Result<(), String> {
    for instance in Installations::load_all_installations().unwrap().0 {
        Installations::add(&instance).unwrap();
    }
    Ok(())
}

#[tauri::command]
pub async fn launch(name: &str) -> Result<(), String> {
    let instance = Installations::find(name).map_err(|e| e.to_string())?;
    instance.execute(None).map_err(|e| e.to_string())?;

    Ok(())
}
