use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader},
    path::PathBuf,
};

use sl_utils::{
    elog,
    errors::{BackendError, InstanceError},
};

use crate::{
    launcher::instances::instance_metadata::InstanceMetadata, INSTANCES_DIR, VERSION_MANIFEST,
};

pub mod game;
pub mod instance_config;
pub mod instance_exporter;
pub mod instance_importer;
pub mod instance_metadata;
pub mod loaded_instance;
pub mod instance_game;

const INSTANCE_FILE_NAME: &str = "instance.json";

fn overwrite_instance(instance_name: &str, metadata: InstanceMetadata) -> std::io::Result<()> {
    let instance_path = INSTANCES_DIR.join(instance_name);
    let instance_file_path = instance_path.join(INSTANCE_FILE_NAME);
    let instance_file = OpenOptions::new().write(true).open(&instance_file_path)?;
    serde_json::to_writer_pretty(instance_file, &metadata)?;
    Ok(())
}

pub async fn edit_instance(
    instance_name: &str,
    new_mc_version: Option<&str>,
    new_modloader_version: Option<&str>,
) -> Result<(), BackendError> {
    let (instance_metadata, _) = self::get_existing(instance_name)?;
    let (mc_version, mc_release_time, mc_release_type) = match new_mc_version {
        Some(new_version) => {
            let version_info = VERSION_MANIFEST
                .get_version_by_id(new_version)
                .ok_or(InstanceError::MinecraftVersionNotFound(new_version.into()))?;

            (
                &version_info.id,
                &version_info.release_time,
                version_info.r#type,
            )
        }
        None => (
            &instance_metadata.mc_version,
            &instance_metadata.mc_release_time,
            instance_metadata.mc_type,
        ),
    };

    let mod_loader = instance_metadata.mod_loader;
    let mod_loader_version = new_modloader_version.unwrap_or(&instance_metadata.mod_loader_version);

    if !mod_loader
        .validate_version(mc_version, mod_loader_version)
        .await?
    {
        return Err(BackendError::InstanceError(
            InstanceError::IncompatibleModLoaderVersion,
        ));
    }

    let new_metadata = InstanceMetadata::new_unchecked(
        instance_metadata.name,
        instance_metadata.icon,
        mc_version.to_string(),
        mc_release_type,
        mc_release_time.to_string(),
        mod_loader,
        mod_loader_version.to_string(),
    );

    self::overwrite_instance(instance_name, new_metadata)?;
    Ok(())
}

/// Renames an instance with the name `instance_name` to `new_name`
pub fn rename_instance(instance_name: &str, new_name: &str) -> Result<(), BackendError> {
    let (mut instance_metadata, old_instance_path) = self::get_existing(instance_name)?;

    let new_instance_path = INSTANCES_DIR.join(new_name);

    std::fs::create_dir_all(&new_instance_path)?;
    sl_utils::fs::copy_dir_all(&old_instance_path, new_instance_path)?;

    instance_metadata.name = new_name.to_string();

    overwrite_instance(new_name, instance_metadata)?;
    std::fs::remove_dir_all(old_instance_path)?;

    Ok(())
}

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
                    Err(BackendError::InstanceError(InstanceError::FailedToExecute(
                        format!("{:?}: {}", path, e),
                    )))
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

    let vec_instances = instances.collect();

    Ok(vec_instances)
}
