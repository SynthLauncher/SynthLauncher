use sl_core::environment::LauncherEnv;
use tokio::sync::RwLock;

use crate::{
    command::{
        launcher::{get_minecraft_versions, open_synthlauncher_root_folder},
        store::cmd_search_store,
    },
    instances::content::get_content_list,
};

use crate::instances::commands::{
    create_instance, delete_instance, edit_instance, get_all_instances, get_instance,
    launch_instance,
};

mod command;
mod instances;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let launcher_env = LauncherEnv::new_at_default();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .manage(RwLock::new(launcher_env))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            /* Launcher */
            open_synthlauncher_root_folder,
            get_minecraft_versions,
            
            /* Instances */
            get_all_instances,
            get_instance,
            launch_instance,
            create_instance,
            get_content_list,
            edit_instance,
            delete_instance,

            
            /* Store */
            cmd_search_store,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
