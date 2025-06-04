mod commands;

use commands::{instances::{get_instances, create_instance, remove_instance}, profiles::{get_current_profile, get_profiles}};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_instances,
            create_instance,
            remove_instance,
            get_current_profile,
            get_profiles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
