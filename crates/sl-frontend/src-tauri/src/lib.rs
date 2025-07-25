use commands::{
    instances::{get_instances, create_instance, remove_instance, launch_instance, load_game_info, kill_instance}, 
    launcher::{open_synthlauncher_folder, open_instance_folder}, 
    minecraft::{get_minecraft_versions},
    store::{search_modrinth_store, search_curseforge_store, get_modrinth_project_versions, install_modrinth_project},
    player_accounts::{create_offline_account, get_accounts, set_current_account}
};
use lazy_static::lazy_static;

use crate::running_instances::RunningInstances;

mod commands;
mod running_instances;

lazy_static! {
    pub static ref RUNNING_INSTANCES: RunningInstances = RunningInstances::new();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_instances,
            create_instance,
            remove_instance,
            open_synthlauncher_folder,
            launch_instance,
            open_instance_folder,
            load_game_info,
            get_minecraft_versions,
            search_modrinth_store,
            search_curseforge_store,
            kill_instance,
            set_current_account,
            get_accounts,
            create_offline_account,
            get_modrinth_project_versions,
            install_modrinth_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
