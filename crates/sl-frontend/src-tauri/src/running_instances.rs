use std::collections::HashMap;

use tokio::{process::Child, sync::Mutex};

#[derive(Debug)]
pub struct RunningInstances {
    pub instances: Mutex<HashMap<String, Child>>,
}

impl RunningInstances {
    pub fn new() -> Self {
        RunningInstances {
            instances: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add(&self, name: String, child: Child) {
        self.instances.lock().await.insert(name, child);
    }

    pub async fn remove(&self, name: &str) {
        self.instances.lock().await.remove(name);
    }

    pub async fn kill(&self, name: &str) {
        let mut processes = self.instances.lock().await;

        if let Some(mut child) = processes.remove(name) {
            let _ = child.kill().await;
        }
    }
}
