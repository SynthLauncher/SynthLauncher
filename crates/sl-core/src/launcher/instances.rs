use std::{fs::{self, OpenOptions}, path::Path};

use serde::{Deserialize, Serialize};
use sl_utils::utils::errors::{BackendError, InstanceError};

use crate::{launcher::instance::Instance, INSTANCES_DIR, INSTANCES_PATH};

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Instances(pub Vec<Instance>);

impl Instances {
    pub fn new() -> Self {
        Instances(Vec::new())
    }

    pub fn load() -> std::io::Result<Self> {
        let content = fs::read_to_string(&INSTANCES_PATH.as_path())?;
        Ok(serde_json::from_str(&content).unwrap_or(Instances::new()))
    }

    pub fn overwrite(instances: &Instances) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(INSTANCES_PATH.as_path())?;

        serde_json::to_writer_pretty(file, &instances)?;

        Ok(())
    }

    pub fn add(instance: &Instance) -> std::io::Result<()> {
        let mut existing_instances = Self::load()?;

        if !existing_instances
            .0
            .iter()
            .any(|existing| existing.name == instance.name)
        {
            existing_instances.0.push(instance.clone());
        }

        Instances::overwrite(&existing_instances)?;

        Ok(())
    }

    pub fn remove(name: &str) -> std::io::Result<()> {
        let mut existing_instances = Self::load()?;

        existing_instances
            .0
            .retain(|existing| existing.name != name);

        Instances::overwrite(&existing_instances)?;

        fs::remove_dir_all(INSTANCES_DIR.join(name))?;

        Ok(())
    }

    fn find_in_instances_dir(name: &str) -> Result<Instance, BackendError> {
        let path = Path::new(&INSTANCES_DIR.as_path()).join(&name);

        if path.exists() && path.is_dir() {
            let instance = Instance::get_instance_from_dir(name)?;
            Instances::add(&instance)?;

            return Ok(instance);
        }

        Err(BackendError::InstanceError(
            InstanceError::InstallationNotFound(name.to_string()),
        ))
    }

    pub fn find(name: &str) -> Result<Instance, BackendError> {
        let instances = Self::load()?;

        if let Some(instance) = instances
            .0
            .into_iter()
            .find(|instance| instance.name == name)
        {
            Ok(instance)
        } else {
            Self::find_in_instances_dir(name)
        }
    }

    pub fn load_all_instances() -> Result<Instances, BackendError> {
        let mut names = Vec::new();
        let mut instances: Instances = Instances(Vec::new());

        for entry in fs::read_dir(INSTANCES_DIR.as_path())? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                if let Some(folder_name_str) = entry_path.file_name().and_then(|f| f.to_str()) {
                    names.push(folder_name_str.to_string());
                }
            }
        }

        for name in names {
            instances.0.push(Instances::find(&name)?);
        }

        Ok(instances)
    }
}
