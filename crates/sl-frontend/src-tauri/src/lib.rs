use sl_core::installations::Installations;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn create_installation() {
    
}

#[tauri::command]
async fn launch() {
    let instance = Installations::find("schoolsmp").unwrap();
    instance.execute(None).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, launch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
