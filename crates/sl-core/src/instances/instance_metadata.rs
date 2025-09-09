use std::{
    fs::OpenOptions,
    io::{Seek, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};
use sl_meta::minecraft::{
    loaders::{
        forge, neoforge,
        quilt::{self},
    },
    version_manifest::{VersionManifest, VersionType},
};
use sl_utils::{
    errors::{BackendError, HttpError, InstanceError},
    requester::Requester,
};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::{
    instances::{
        instance_exporter::InstanceExporter, loaded_instance::LoadedInstance, InstanceManager
    },
    minecraft::minecraft_version::MinecraftVersionID,
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
        requester: &Requester,
        mc_version: &str,
        mod_loader_version: &str,
    ) -> Result<bool, HttpError> {
        let do_request = async |url: &str| -> Result<_, HttpError> {
            Ok(requester.builder().download(url).await?.to_vec())
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

    pub async fn get_latest_version(
        &self,
        requester: &Requester,
        mc_version: &str,
    ) -> Result<String, HttpError> {
        let do_request = async |url: &str| -> Result<_, HttpError> {
            Ok(requester.builder().download(url).await?.to_vec())
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
pub struct InstanceMetadata {
    pub scheme_version: u32,
    pub name: String,
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
        mc_version: String,
        mc_type: VersionType,
        mc_release_time: String,
        mod_loader: ModLoader,
        mod_loader_version: String,
    ) -> Self {
        Self {
            scheme_version: 0,
            name,
            mc_version,
            mc_release_time,
            mc_type,
            mod_loader_version,
            mod_loader,
        }
    }

    pub(crate) async fn new(
        requester: &Requester,
        version_manifest: &VersionManifest,
        name: String,
        mc_version: &str,
        mod_loader: ModLoader,
        mod_loader_version: Option<String>,
    ) -> Result<Self, BackendError> {
        let version =
            version_manifest
                .get_version_by_id(mc_version)
                .ok_or(BackendError::InstanceError(
                    InstanceError::MinecraftVersionNotFound(mc_version.to_string()),
                ))?;

        let mod_loader_version = match mod_loader_version {
            Some(specific) => specific,
            None => mod_loader.get_latest_version(requester, mc_version).await?,
        };

        Ok(Self::new_unchecked(
            name,
            version.id.clone(),
            version.r#type.clone(),
            version.release_time.clone(),
            mod_loader,
            mod_loader_version,
        ))
    }
    
    pub async fn get_instance_icon<'a>(&self, man: &'a InstanceManager<'a>) -> Option<Vec<u8>> {
        let instance_dir = man.instance_dir(&self.name);
        let icon_path = instance_dir.join("icon.png");
        tokio::fs::read(icon_path).await.ok()
    }

    /// Loads ('Upgrades' information to) an instance's in memory representation
    pub async fn load_init<'a>(
        self,
        man: &'a InstanceManager<'a>,
    ) -> Result<LoadedInstance<'a>, BackendError> {
        let instance_dir = man.instance_dir(&self.name);

        let version_id = MinecraftVersionID::new(
            self.mod_loader,
            self.mod_loader_version.clone(),
            self.mc_version.clone(),
        );

        let (loaded_version, config) = version_id.load_init(man, &instance_dir).await?;

        Ok(LoadedInstance::new(
            man,
            self,
            instance_dir,
            loaded_version,
            config,
        ))
    }

    /// Creates an instance exporter that will export the instance to a Writer in Zip format
    pub fn exporter<'a, W: Write + Seek>(
        self,
        instance_man: &InstanceManager<'_>,
        export_to: W,
    ) -> InstanceExporter<'a, W> {
        InstanceExporter::new(export_to, instance_man.instance_dir(&self.name))
    }

    /// Creates an Instance Exporter that will export the instance to a given Path, the exported data would be in Zip format
    pub fn exporter_to_path<'a>(
        self,
        instance_man: &InstanceManager<'_>,
        path: &Path,
    ) -> std::io::Result<InstanceExporter<'a, impl Write + Seek>> {
        let export_to_file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        Ok(Self::exporter(self, instance_man, export_to_file))
    }
}
