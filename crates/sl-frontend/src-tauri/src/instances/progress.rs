use std::{path::PathBuf, sync::atomic::{AtomicBool, AtomicUsize, Ordering}, time::Duration};

use serde::{Deserialize, Serialize};
use sl_core::sl_utils::{elog, progress::{ProgressEvent, ProgressReceiver, ProgressReport}};
use tauri::{AppHandle, Emitter, Listener, Runtime, WebviewUrl, WebviewWindow};


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
    .title("Downloading...")
    .maximizable(false)
    .center()
    .closable(false)
    .inner_size(600., 300.)
    .devtools(true)
    .decorations(true)
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
