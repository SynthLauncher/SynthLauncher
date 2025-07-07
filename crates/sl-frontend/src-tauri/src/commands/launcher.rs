use sl_core::{launcher::addons::get_addons, INSTANCES_DIR, LAUNCHER_DIR};
use std::{path::Path, process::Command};

pub fn open_folder(folder_path: &Path) {
    let result = if cfg!(target_os = "windows") {
        Command::new("explorer").arg(folder_path).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(folder_path).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(folder_path).spawn()
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported OS",
        ))
    };

    if let Err(e) = result {
        eprintln!("Failed to open folder: {}", e);
    }    
}

#[tauri::command]
pub async fn open_synthlauncher_folder() {
    let folder_path = &*LAUNCHER_DIR;
    open_folder(&folder_path);
}

#[tauri::command]
pub async fn open_instance_folder(name: String) {
    let folder_path = &*INSTANCES_DIR.join(name);
    open_folder(&folder_path);
}

#[tauri::command]
pub async fn get_synthlauncher_addons() -> Result<Vec<String>, String> {
    let addons = get_addons().map_err(|e| e.to_string())?;

    Ok(addons)
}

