use std::{
    fs::OpenOptions,
    io::{Seek, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sl_meta::minecraft::{
    loaders::{
        forge, neoforge,
        quilt::{self},
    },
    version_manifest::VersionType,
};
use sl_utils::errors::{BackendError, HttpError, InstanceError};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::{
    launcher::{
        instances::{self, instance_exporter::InstanceExporter, loaded_instance::LoadedInstance},
        minecraft_version::MinecraftVersionID,
    },
    INSTANCES_DIR, REQUESTER, VERSION_MANIFEST,
};

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    EnumString,
    Display,
    AsRefStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum ModLoader {
    #[default]
    Vanilla,
    Fabric,
    Quilt,
    Forge,
    NeoForge,
}

impl ModLoader {
    /// Validate that the combination of Minecraft version and mod loader version is valid.
    pub async fn validate_version(
        &self,
        mc_version: &str,
        mod_loader_version: &str,
    ) -> Result<bool, HttpError> {
        let do_request = async |url: &str| -> Result<_, HttpError> {
            Ok(REQUESTER.builder().download(url).await?.to_vec())
        };

        match self {
            Self::Vanilla => Ok(false),
            Self::Fabric => {
                let versions = sl_meta::minecraft::loaders::fabric::versions::get_fabric_versions(
                    mc_version, do_request,
                )
                .await?;

                let is_valid = versions
                    .iter()
                    .any(|f| f.loader.version == mod_loader_version);
                Ok(is_valid)
            }
            Self::Quilt => {
                let versions = sl_meta::minecraft::loaders::quilt::versions::get_quilt_versions(
                    mc_version, do_request,
                )
                .await?;

                let is_valid = versions
                    .iter()
                    .any(|f| f.loader.version == mod_loader_version);
                Ok(is_valid)
            }
            Self::Forge => {
                let versions = forge::ForgeVersions::download(do_request).await?;
                Ok(versions.validate(mc_version, mod_loader_version))
            }
            Self::NeoForge => {
                let versions = neoforge::NeoForgeReleases::download(do_request).await?;
                Ok(versions.validate(mc_version, mod_loader_version))
            }
        }
    }

    pub async fn get_latest_version(&self, mc_version: &str) -> Result<String, HttpError> {
        let do_request = async |url: &str| -> Result<_, HttpError> {
            Ok(REQUESTER.builder().download(url).await?.to_vec())
        };

        match self {
            Self::Vanilla => Ok(String::new()),
            Self::Quilt => quilt::versions::get_latest_loader_version(mc_version, do_request).await,
            Self::Fabric => {
                sl_meta::minecraft::loaders::fabric::versions::get_latest_loader_version(
                    mc_version, do_request,
                )
                .await
            }

            Self::Forge => Ok(forge::ForgeVersions::download(do_request)
                .await?
                .get_latest_forge_version(mc_version)
                .expect("no forge version were found for this minecraft version")
                .to_string()),
            Self::NeoForge => Ok(neoforge::NeoForgeReleases::download(do_request)
                .await?
                .latest_from_mc_version(mc_version)
                .expect("no neoforge version were found for this minecraft version")
                .to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameVersionMetadata {
    #[serde(rename = "id")]
    pub version: String,
    pub release_time: String,
    pub r#type: VersionType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstanceMetadata {
    pub name: String,
    pub icon: Option<String>,
    pub mc_version: String,
    pub mc_release_time: String,
    pub mc_type: VersionType,
    pub mod_loader_version: String,
    pub mod_loader: ModLoader,
}

impl InstanceMetadata {
    /// Constructs a new instance metadata, without doing any version checks.
    pub(super) const fn new_unchecked(
        name: String,
        icon: Option<String>,
        mc_version: String,
        mc_type: VersionType,
        mc_release_time: String,
        mod_loader: ModLoader,
        mod_loader_version: String,
    ) -> Self {
        Self {
            name,
            icon,
            mc_version,
            mc_release_time,
            mc_type,
            mod_loader_version,
            mod_loader,
        }
    }

    async fn new(
        name: String,
        mc_version: &str,
        mod_loader: ModLoader,
        mod_loader_version: Option<String>,
        icon: Option<String>,
    ) -> Result<Self, BackendError> {
        let version =
            VERSION_MANIFEST
                .get_version_by_id(mc_version)
                .ok_or(BackendError::InstanceError(
                    InstanceError::MinecraftVersionNotFound(mc_version.to_string()),
                ))?;

        // TODO: remove this line
        std::fs::create_dir_all(INSTANCES_DIR.join(&name))?;

        let mod_loader_version = match mod_loader_version {
            Some(specific) => specific,
            None => mod_loader.get_latest_version(mc_version).await?,
        };

        Ok(Self::new_unchecked(
            name,
            icon,
            version.id.clone(),
            version.r#type.clone(),
            version.release_time.clone(),
            mod_loader,
            mod_loader_version,
        ))
    }

    /// Creates a new instance, and adds it to the instances list at once
    pub async fn create(
        name: String,
        version: &str,
        mod_loader: ModLoader,
        mod_loader_version: Option<String>,
        icon: Option<String>,
    ) -> Result<Self, BackendError> {
        let instance = Self::new(name, version, mod_loader, mod_loader_version, icon).await?;
        // TODO: embed this into this struct for cleaner code
        instances::add_new(&instance)?;
        Ok(instance)
    }

    fn instance_dir(&self) -> PathBuf {
        INSTANCES_DIR.join(&self.name)
    }

    /// Loads ('Upgrades' information to) an instance's in memory representation
    pub async fn load_init(self) -> Result<LoadedInstance, BackendError> {
        let instance_dir = self.instance_dir();

        let version_id = MinecraftVersionID::new(
            self.mod_loader,
            self.mod_loader_version.clone(),
            self.mc_version.clone(),
        );

        let (loaded_version, config) = version_id.load_init(&instance_dir).await?;

        Ok(LoadedInstance::new(
            self,
            instance_dir,
            loaded_version,
            config,
        ))
    }

    /// Creates an instance exporter that will export the instance to a Writer in Zip format
    pub fn exporter<'a, W: Write + Seek>(self, export_to: W) -> InstanceExporter<'a, W> {
        InstanceExporter::new(export_to, self.instance_dir())
    }

    /// Creates an Instance Exporter that will export the instance to a given Path, the exported data would be in Zip format
    pub fn exporter_to_path<'a>(
        self,
        path: &Path,
    ) -> std::io::Result<InstanceExporter<'a, impl Write + Seek>> {
        let export_to_file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        Ok(Self::exporter(self, export_to_file))
    }
}
