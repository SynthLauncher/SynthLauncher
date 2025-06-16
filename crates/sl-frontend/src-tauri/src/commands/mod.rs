use std::process::Command;

use sl_core::LAUNCHER_DIR;

pub mod instances;
pub mod profiles;
pub mod store;

#[tauri::command]
pub async fn open_synthlauncher_folder() {
    let folder_path = &*LAUNCHER_DIR;

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
