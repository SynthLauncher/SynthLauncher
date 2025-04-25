use sl_core::json::manifest::manifest_read;
use sl_meta::json::version_manifest::Version;

#[tauri::command]
fn get_versions() -> Vec<Version> {
    let manifest = manifest_read();
    let versions: Vec<Version> = manifest.versions().cloned().collect();
    return versions;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_versions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
