use synthlauncher_backend::config::java::JavaInstallation;

#[tauri::command]
fn my_custom_command() -> Result<String, ()> {
  let result = JavaInstallation::get_installations();
  match result {
    Ok(res) => Ok(serde_json::to_string(&res).unwrap()),
    Err(err) => Ok(err.to_string()),
  }
}


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
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
