// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sl_core::launcher::init_launcher_dir;
use sl_utils::errors::BackendError;
use tokio::runtime::Runtime;

fn main() -> Result<(), BackendError> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        init_launcher_dir().await?;
        Ok::<(), BackendError>(())
    })?;

    sl_frontend_lib::run();
    Ok(())
}