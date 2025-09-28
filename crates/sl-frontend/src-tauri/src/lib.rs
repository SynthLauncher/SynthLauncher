use sl_core::environment::LauncherEnv;
use tokio::sync::RwLock;

use crate::command::{
    instances::{get_all_instances, get_instance, launch_instance}, launcher::open_synthlauncher_root_folder,
    store::fetch_store_search,
};

pub mod command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let launcher_env = LauncherEnv::new_at_default();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .manage(RwLock::new(launcher_env))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            open_synthlauncher_root_folder,
            fetch_store_search,
            get_all_instances,
            get_instance,
            launch_instance
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
