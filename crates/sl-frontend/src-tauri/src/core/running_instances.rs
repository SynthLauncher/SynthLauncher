use lazy_static::lazy_static;
use std::collections::HashSet;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

lazy_static! {
    pub static ref RUNNING_INSTANCES: RunningInstances = RunningInstances::new();
}

#[derive(Debug)]
pub struct RunningInstances {
    pub instances: RwLock<HashSet<String>>,
}

impl RunningInstances {
    pub fn new() -> Self {
        RunningInstances {
            instances: RwLock::new(HashSet::new()),
        }
    }

    /// Add a new instance to the running instances list.
    pub async fn add(&self, name: String, app_handle: &AppHandle) {
        self.instances.write().await.insert(name.clone());
        let _ = app_handle.emit("running_instances_updates", name);
    }

    /// Remove an instance from the running instances list.
    pub async fn remove(&self, name: &str, app_handle: &AppHandle) -> bool {
        let removed = self.instances.write().await.remove(name);
        if removed {
            let _ = app_handle.emit("running_instances_updates", name.to_string());
        }

        removed
    }

    /// Check if an instance is alive.
    pub async fn is_alive(&self, name: &str) -> bool {
        self.instances.read().await.contains(name)
    }

    pub async fn list(&self) -> Vec<String> {
        self.instances.read().await.iter().cloned().collect()
    }
}

