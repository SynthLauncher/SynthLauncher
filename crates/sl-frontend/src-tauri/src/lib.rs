mod commands;

use commands::{instances::{get_instances, test_progress, create_instance, remove_instance}, store::search_store, profiles::{get_current_profile, get_profiles, get_other_profiles}, launcher::open_synthlauncher_folder, minecraft::get_minecraft_versions};

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
            get_profiles,
            get_other_profiles,
            open_synthlauncher_folder,
            get_minecraft_versions,
            test_progress,
            search_store
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
