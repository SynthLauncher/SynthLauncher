mod commands;
use commands::{
    create_installation, edit_username, get_installations, get_username, 
    load_all_installations, remove_installation,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // launch,
            get_username,
            edit_username,
            get_installations,
            create_installation,
            remove_installation,
            load_all_installations
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
