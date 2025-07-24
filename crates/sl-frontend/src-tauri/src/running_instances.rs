use std::collections::HashSet;
use tokio::sync::RwLock;

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
    pub async fn add(&self, name: String) {
        self.instances.write().await.insert(name);
    }
    /// Remove an instance from the running instances list.
    pub async fn remove(&self, name: &str) -> bool {
        self.instances.write().await.remove(name)
    }
    /// Check if an instance is alive.
    pub async fn is_alive(&self, name: &str) -> bool {
        self.instances.read().await.contains(name)
    }
}
