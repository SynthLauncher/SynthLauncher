use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::PathBuf,
};

use sl_utils::{
    elog,
    utils::errors::{BackendError, InstanceError},
};

use crate::{launcher::instances::metadata::InstanceMetadata, INSTANCES_DIR};

pub mod instance;
pub mod metadata;
pub mod game;

const INSTANCE_FILE_NAME: &str = "instance.json";

/// Gets an existing instance by name assuming it may not exist
/// returns Ok(None) if it does not exist
pub(super) fn find(name: &str) -> std::io::Result<Option<(InstanceMetadata, PathBuf)>> {
    // FIXME: maybe don't rely on the name for getting an existing instance's path
    let instance_file_path = INSTANCES_DIR.join(name).join(INSTANCE_FILE_NAME);
    let instance_file = match File::open(&instance_file_path) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    };

    let read_buf = BufReader::new(instance_file);
    let instance = serde_json::from_reader(read_buf)?;

    Ok(Some((instance, instance_file_path)))
}

pub(super) fn add_new(instance: &InstanceMetadata) -> Result<(), BackendError> {
    if self::find(&instance.name)?.is_some() {
        return Err(BackendError::InstanceError(
            InstanceError::InstanceAlreadyExists(instance.name.clone()),
        ));
    }

    let new_instance_file_path = INSTANCES_DIR.join(&instance.name).join(INSTANCE_FILE_NAME);

    if let Some(parent) = new_instance_file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let instance_file = File::create_new(&new_instance_file_path)?;
    serde_json::to_writer_pretty(instance_file, &instance)?;

    Ok(())
}

pub fn remove(name: &str) -> Result<(), BackendError> {
    let (_, instance_file_path) = self::get_existing(name)?;

    if let Some(parent) = instance_file_path.parent() {
        fs::remove_dir_all(parent)?;
    }
    Ok(())
}

/// Gets an existing instance by name assuming it exists
/// errors if it does not exist
pub fn get_existing(name: &str) -> Result<(InstanceMetadata, PathBuf), BackendError> {
    self::find(name)?.ok_or_else(|| {
        BackendError::InstanceError(InstanceError::InstanceNotFound(name.to_string()))
    })
}

/// Gets all instances information from the instances directory
pub fn get_all_instances() -> Result<Vec<InstanceMetadata>, BackendError> {
    let instances_dir = INSTANCES_DIR.read_dir()?;

    let instances_paths = instances_dir
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_dir()))
        .map(|entry| entry.path())
        .map(|path| path.join(INSTANCE_FILE_NAME))
        .filter(|instance_file_path| instance_file_path.exists());

    let instances = instances_paths
        .map(|path| -> Result<_, BackendError> {
            let instance_file = File::open(&path)?;
            let deserialized: Result<InstanceMetadata, _> = serde_json::from_reader(instance_file);
            match deserialized {
                Ok(instance) => Ok(instance),
                Err(e) => {
                    elog!("Failed to load instance at {:?}: {}. Skipping.", path, e);
                    Err(BackendError::InstanceError(InstanceError::FailedToExecute(format!("{:?}: {}", path, e))))
                }
            }
        })
        .filter_map(|instance| match instance {
            Err(e) => {
                elog!("failed to load an instance error: {}, ignoring...", e);
                None
            }
            Ok(i) => Some(i),
        });

    Ok(instances.collect())
}
