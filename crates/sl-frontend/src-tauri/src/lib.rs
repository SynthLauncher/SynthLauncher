use commands::{
    instances::{get_instances, create_instance, remove_instance, launch_instance, kill_instance, get_running_instances, export_instance}, 
    launcher::{open_synthlauncher_folder, open_instance_folder, open_folder}, 
    minecraft::{get_minecraft_versions},
    store::{search_modrinth_store, search_curseforge_store, get_modrinth_project_versions, install_modrinth_project},
    accounts::{accounts_get, accounts_set_current, accounts_remove, accounts_create_offline}
};

mod commands;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            accounts_get,
            accounts_set_current,
            accounts_remove,
            accounts_create_offline,

            get_instances,
            create_instance,
            remove_instance,
            launch_instance,
            export_instance,
            kill_instance,
            get_running_instances,
            
            open_folder,
            open_synthlauncher_folder,
            open_instance_folder,
  
            get_minecraft_versions,
            search_modrinth_store,
            search_curseforge_store,
            get_modrinth_project_versions,
            install_modrinth_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
