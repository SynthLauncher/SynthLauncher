use std::{
    path::PathBuf,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use sl_core::{
    environment::LauncherEnv,
    instances::instance_metadata::InstanceMetadata,
    sl_utils::{
        elog,
        progress::{ProgressEvent, ProgressReceiver, ProgressReport},
    },
};
use tauri::{AppHandle, Emitter, Listener, Runtime, State, WebviewUrl, WebviewWindow};
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize)]
pub struct Instance {
    #[serde(flatten)]
    metadata: InstanceMetadata,
    icon: Option<Vec<u8>>,
}

#[tauri::command]
pub async fn get_instance(
    name: &str,
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<Instance, String> {
    let env = launcher_env.read().await;

    let instance_metadata = env
        .instances()
        .get_existing(name)
        .map_err(|e| e.to_string())?
        .0;
    let icon = instance_metadata.get_instance_icon(&env.instances()).await;

    Ok(Instance {
        metadata: instance_metadata,
        icon: icon,
    })
}

#[tauri::command]
pub async fn get_all_instances(
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<Vec<Instance>, String> {
    let mut instances = Vec::new();
    let env = launcher_env.read().await;

    let instance_metadatas = env
        .instances()
        .get_all_instances()
        .await
        .map_err(|e| e.to_string())?;

    for metadata in instance_metadatas {
        let icon = metadata.get_instance_icon(&env.instances()).await;
        instances.push(Instance {
            metadata: metadata,
            icon: icon,
        });
    }

    Ok(instances)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    url: String,
    total_bytes: u64,
    current_bytes: u64,
}
impl Progress {
    pub fn from_report(report: ProgressReport) -> Self {
        Self {
            url: report.url().to_string(),
            total_bytes: report.total(),
            current_bytes: report.current(),
        }
    }
}

static CURR_PROGRESS_WIN_NUMBER: AtomicUsize = AtomicUsize::new(0);

pub async fn create_progress_window<R: Runtime>(app_handle: &AppHandle<R>) -> WebviewWindow<R> {
    static WINDOW_CREATION_LOCK: AtomicBool = AtomicBool::new(false);
    static WINDOW_READY: AtomicBool = AtomicBool::new(false);

    while WINDOW_CREATION_LOCK
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let label = format!(
        "progress-{}",
        CURR_PROGRESS_WIN_NUMBER.fetch_add(1, Ordering::Relaxed)
    );
    let window = tauri::WebviewWindowBuilder::new(
        app_handle,
        label,
        WebviewUrl::App(PathBuf::from("progress")),
    )
    .maximizable(false)
    .center()
    .closable(false)
    .inner_size(600., 300.)
    .devtools(true)
    .decorations(false)
    .build()
    .expect("Failed to construct progress window");

    WINDOW_READY.store(false, Ordering::Release);
    window.once("progress-ready", |_| {
        WINDOW_READY.store(true, Ordering::Release);
    });

    while !WINDOW_READY.load(Ordering::Acquire) {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    WINDOW_CREATION_LOCK.store(false, Ordering::Release);
    window
}

pub fn progress_receiver<R: Runtime>(w: WebviewWindow<R>) -> ProgressReceiver {
    ProgressReceiver::new(move |event| {
        let results = match event {
            ProgressEvent::Begin(msg) => w.emit("progress-begin", msg),
            ProgressEvent::Progress(report) => {
                w.emit("progress-update", Progress::from_report(report))
            }
            ProgressEvent::End => w.emit("progress-done", ()),
            ProgressEvent::StopAll => w.destroy(),
            ProgressEvent::StopForUrl(url) => w.emit("progress-stop", url),
        };

        if let Err(err) = results {
            elog!("Error emitting progress event: {err:#?}");
        }
    })
}
#[tauri::command]
pub async fn launch_instance(
    app_handle: AppHandle,
    name: &str,
    launcher_env: State<'_, RwLock<LauncherEnv>>,
) -> Result<(), String> {
    let env = launcher_env.read().await;

    let instance_metadata = env
        .instances()
        .get_existing(name)
        .map_err(|e| e.to_string())?
        .0;

    let window = create_progress_window(&app_handle).await;
    let receiver = progress_receiver(window);
    instance_metadata
        .load_init(&env.instances())
        .await
        .map_err(|e| e.to_string())?
        .execute(receiver)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
