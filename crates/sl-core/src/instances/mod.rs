use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader, Read, Seek},
    path::{Path, PathBuf},
};

use crate::{
    accounts::PlayerAccounts, environment::LauncherEnv, instances::instance_metadata::ModLoader,
};
use sl_java_manager::jre_manifest::JreManifest;
use sl_meta::minecraft::version_manifest::VersionManifest;
use sl_utils::{
    elog,
    errors::{BackendError, InstanceError, InstanceImportErr},
    requester::Requester,
};

use crate::instances::instance_metadata::InstanceMetadata;

pub mod content_caching;
pub mod instance_exporter;
pub mod instance_importer;
pub mod instance_metadata;
pub mod loaded_instance;

const INSTANCE_FILE_NAME: &str = "instance.json";

/// Manages instances
pub struct InstanceManager<'a> {
    env: &'a LauncherEnv,
}

impl<'a> InstanceManager<'a> {
    pub(crate) fn new(env: &'a LauncherEnv) -> Self {
        Self { env }
    }

    /// Creates a new instance, returning it's metadata
    pub async fn create_instance(
        &mut self,
        name: String,
        version: &str,
        mod_loader: ModLoader,
        mod_loader_version: Option<String>,
    ) -> Result<InstanceMetadata, BackendError> {
        let instance = InstanceMetadata::new(
            self.requester(),
            self.version_manifest().await,
            name,
            version,
            mod_loader,
            mod_loader_version,
        )
        .await?;
        self.add_new(&instance)?;
        Ok(instance)
    }

    /// Imports an Instance exported in a Zip format, from a reader
    pub async fn import_instance<R: Read + Seek>(
        &mut self,
        reader: R,
    ) -> Result<(), InstanceImportErr> {
        instance_importer::import_instance(self, reader).await
    }

    /// Imports an Instance exported in a Zip format from a file at `file_path`
    pub async fn import_instance_from_path(
        &mut self,
        file_path: &Path,
    ) -> Result<(), InstanceImportErr> {
        instance_importer::import_instance_from_path(self, file_path).await
    }

    pub async fn version_manifest(&self) -> &'a VersionManifest {
        self.env.version_manifest().await
    }

    pub async fn jre_manifest(&self) -> &'a JreManifest {
        self.env.jre_manifest().await
    }

    pub const fn requester(&self) -> &'a Requester {
        self.env.requester()
    }

    pub fn dir(&self) -> &'a Path {
        self.env.instances_dir()
    }

    pub fn javas_path(&self) -> &'a Path {
        self.env.javas_path()
    }

    /// Returns the path to the instance directory under a given name
    pub fn instance_dir(&self, instance_name: &str) -> PathBuf {
        self.dir().join(instance_name)
    }
    /// Returns the path to the instance file under a given name
    pub fn instance_file(&self, instance_name: &str) -> PathBuf {
        self.instance_dir(instance_name).join(INSTANCE_FILE_NAME)
    }

    pub fn assets_root(&self) -> &'a Path {
        self.env.assets_path()
    }

    pub fn libs_root(&self) -> &'a Path {
        self.env.libs_path()
    }

    pub fn versions_root(&self) -> &'a Path {
        self.env.versions_path()
    }

    pub const fn config(&self) -> &'a config::Config {
        self.env.config()
    }

    /// Attempts to load the accounts from the environment.
    pub async fn try_load_accounts(&self) -> io::Result<PlayerAccounts> {
        self.env.accounts().load().await
    }

    fn overwrite_instance(
        &mut self,
        instance_name: &str,
        metadata: InstanceMetadata,
    ) -> std::io::Result<()> {
        let instance_path = self.instance_dir(instance_name);
        let instance_file_path = instance_path.join(INSTANCE_FILE_NAME);

        let instance_file = OpenOptions::new().write(true).open(&instance_file_path)?;

        serde_json::to_writer_pretty(instance_file, &metadata)?;
        Ok(())
    }

    /// Edits an existing instance, changes its minecraft version or modloader version or both
    pub async fn edit_instance(
        &mut self,
        instance_name: &str,
        new_mc_version: Option<&str>,
        new_modloader_version: Option<&str>,
    ) -> Result<(), BackendError> {
        let (instance_metadata, _) = self.get_existing(instance_name)?;
        let (mc_version, mc_release_time, mc_release_type) = match new_mc_version {
            Some(new_version) => {
                let version_info = self
                    .version_manifest()
                    .await
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
        let mod_loader_version =
            new_modloader_version.unwrap_or(&instance_metadata.mod_loader_version);

        if !mod_loader
            .validate_version(self.requester(), mc_version, mod_loader_version)
            .await?
        {
            return Err(BackendError::InstanceError(
                InstanceError::IncompatibleModLoaderVersion,
            ));
        }

        let new_metadata = InstanceMetadata::new_unchecked(
            instance_metadata.name,
            mc_version.to_string(),
            mc_release_type,
            mc_release_time.to_string(),
            mod_loader,
            mod_loader_version.to_string(),
        );

        self.overwrite_instance(instance_name, new_metadata)?;
        Ok(())
    }

    /// Renames an instance with the name `instance_name` to `new_name`
    pub async fn rename_instance(
        &mut self,
        instance_name: &str,
        new_name: &str,
    ) -> Result<(), BackendError> {
        let (mut instance_metadata, old_instance_path) = self.get_existing(instance_name)?;

        let new_instance_path = self.instance_dir(new_name);

        tokio::fs::create_dir_all(&new_instance_path).await?;
        sl_utils::fs::async_copy_dir_all(&old_instance_path, new_instance_path).await?;

        instance_metadata.name = new_name.to_string();

        self.overwrite_instance(new_name, instance_metadata)?;
        tokio::fs::remove_dir_all(old_instance_path).await?;

        Ok(())
    }

    #[inline]
    /// Gets an existing instance by name assuming it may not exist
    /// returns Ok(None) if it does not exist
    pub(super) fn find(&self, name: &str) -> std::io::Result<Option<(InstanceMetadata, PathBuf)>> {
        // FIXME: maybe don't rely on the name for getting an existing instance's path
        let instance_file_path = self.instance_file(name);
        let instance_file = match File::open(&instance_file_path) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err),
        };

        let read_buf = BufReader::new(instance_file);
        let instance = serde_json::from_reader(read_buf)?;

        Ok(Some((instance, instance_file_path)))
    }

    pub(super) fn add_new(&mut self, instance: &InstanceMetadata) -> Result<(), BackendError> {
        if self.find(&instance.name)?.is_some() {
            return Err(BackendError::InstanceError(
                InstanceError::InstanceAlreadyExists(instance.name.clone()),
            ));
        }

        let new_instance_file_path = self.instance_file(&instance.name);
        if let Some(parent) = new_instance_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let instance_file = File::create_new(&new_instance_file_path)?;
        serde_json::to_writer_pretty(instance_file, &instance)?;

        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<(), BackendError> {
        let (_, instance_file_path) = self.get_existing(name)?;

        if let Some(parent) = instance_file_path.parent() {
            fs::remove_dir_all(parent)?;
        }
        Ok(())
    }

    /// Gets an existing instance by name assuming it exists
    /// errors if it does not exist
    pub fn get_existing(&self, name: &str) -> Result<(InstanceMetadata, PathBuf), BackendError> {
        self.find(name)?.ok_or_else(|| {
            BackendError::InstanceError(InstanceError::InstanceNotFound(name.to_string()))
        })
    }

    /// Gets all instances information from the instances directory
    pub async fn get_all_instances(&self) -> Result<Vec<InstanceMetadata>, BackendError> {
        let mut instances_dir = tokio::fs::read_dir(self.dir()).await?;

        let mut instances: Vec<InstanceMetadata> = Vec::new();
        while let Some(entry) = instances_dir.next_entry().await? {
            if !entry.file_type().await.is_ok_and(|ft| ft.is_dir()) {
                continue;
            }

            let instance_dir_path = entry.path();
            let instance_file_path = instance_dir_path.join(INSTANCE_FILE_NAME);
            if !instance_file_path.exists() {
                continue;
            }

            let load_instance_known = async || -> Result<InstanceMetadata, BackendError> {
                let instance_file_data = tokio::fs::read(&instance_file_path).await?;
                let instance = serde_json::from_slice(&instance_file_data)?;
                Ok(instance)
            };

            match load_instance_known().await {
                Ok(instance) => instances.push(instance),
                Err(err) => {
                    elog!(
                        "Failed to load instance at {}: {err}. Skipping.",
                        instance_dir_path.display(),
                    );
                }
            }
        }

        Ok(instances)
    }
}
