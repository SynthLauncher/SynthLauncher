use std::sync::Arc;

use sl_core::environment::LauncherEnv;
use tokio::sync::RwLock;

use crate::command::launcher::open_synthlauncher_root_folder;


pub mod command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let launcher_env = LauncherEnv::new_at_default();
    let shared_launcher_env = Arc::new(RwLock::new(launcher_env));

    tauri::Builder::default()
        .manage(shared_launcher_env)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_synthlauncher_root_folder            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
