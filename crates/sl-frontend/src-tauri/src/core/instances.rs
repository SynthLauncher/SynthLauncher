use data_encoding::BASE32_NOPAD;
use sl_core::launcher::instances;
use sl_utils::errors::BackendError;
use tauri::{AppHandle, Emitter};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    select,
    time::{sleep, Duration},
};

use crate::core::running_instances::RUNNING_INSTANCES;

pub async fn launch_instance_inner(name: &str, app_handle: AppHandle) -> Result<(), BackendError> {
    let encoded = BASE32_NOPAD.encode(name.as_bytes());
    let emit_target = format!("{encoded}-console");

    let (instance, _) = instances::get_existing(name)?;
    let loaded_instance = instance.load_init().await?;

    let (mut child, reader) = loaded_instance.execute().await?;
    let mut reader = BufReader::new(reader);

    RUNNING_INSTANCES.add(name.to_string(), &app_handle).await;

    let mut line = String::new();

    let emit = |line: &str| app_handle.emit(&emit_target, line);
    
    emit("Starting instance...")
        .expect("failed to emit the initial data to the instance's Console");

    let mut dead_peacfully = false;

    loop {
        select! {
            read_result = reader.read_line(&mut line) => {
                match read_result {
                    Ok(0) => {
                        if let Ok(Some(status)) = child.try_wait() {
                            emit(&format!("Exited with code: {}\n", status.code().unwrap_or(-1)))
                                .expect("failed to emit end");
                            dead_peacfully = true;
                            break 
                        }
                    },
                    Ok(_) => {
                        let _ = emit(&line);
                        line.clear();
                    }
                    Err(_) => break,
                }
            }

            _ = sleep(Duration::from_millis(300)) => {
                if !RUNNING_INSTANCES.is_alive(name).await {
                    break;
                }
            }
        }
    }

    RUNNING_INSTANCES.remove(&name, &app_handle).await;

    // in case it was removed from the list or an error occurred
    if !dead_peacfully {
        _ = child.kill().await;
        emit("The instance has been killed!\n").expect("failed to emit end data");
    }

    Ok(())
}
