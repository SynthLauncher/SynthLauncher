use commands::{
    instances::{get_instances, create_instance, remove_instance, launch_instance, load_game_info}, 
    profiles::{get_current_profile, get_profiles, get_other_profiles}, 
    launcher::{open_synthlauncher_folder, open_instance_folder}, 
    minecraft::{get_minecraft_versions},
    store::{search_modrinth_store, search_curseforge_store}
};

mod commands;

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
            launch_instance,
            open_instance_folder,
            load_game_info,
            get_minecraft_versions,
            search_modrinth_store,
            search_curseforge_store
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
