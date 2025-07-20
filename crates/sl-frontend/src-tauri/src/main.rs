// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sl_core::launcher::init_launcher_dir;
use sl_utils::errors::BackendError;
use tokio::runtime::Runtime;

fn config_display_env() {
    let session = std::env::var("XDG_SESSION_TYPE")
        .expect("Expected XDG_SESSION_TYPE env var!");

    if session == "x11" {
        std::env::set_var("DISPLAY", ":0");
        std::env::remove_var("WAYLAND_DISPLAY");
    } else if session == "wayland" {
        std::env::remove_var("DISPLAY");
    }

    std::env::set_var("LIBGL_ALWAYS_SOFTWARE", "1");
}

fn main() -> Result<(), BackendError> {
    config_display_env();
    
    let rt = Runtime::new()?;
    rt.block_on(async {
        init_launcher_dir().await?;
        Ok::<(), BackendError>(())
    })?;

    sl_frontend_lib::run();
    Ok(())
}

