use sl_core::environment::LauncherEnv;
use tokio::sync::Mutex;

use crate::command::launcher::open_synthlauncher_root_folder;

pub mod command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let launcher_env = LauncherEnv::new_at_default();

    tauri::Builder::default()
        .manage(Mutex::new(launcher_env))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            open_synthlauncher_root_folder            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
