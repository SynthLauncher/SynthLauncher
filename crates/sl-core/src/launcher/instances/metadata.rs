use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sl_meta::minecraft::{loaders::vanilla::Client, version_manifest::VersionType};
use sl_utils::{
    dlog,
    utils::errors::{BackendError, InstanceError},
};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::{
    launcher::{
        config::{self, InstanceConfig},
        instances::{self, instance::LoadedInstance},
    },
    loaders::{
        fabric::install_fabric_loader, forge::install_forge_loader,
        neoforge::install_neoforge_loader, quilt::install_quilt_loader, Loaders,
    },
    minecraft::version_manifest::download_version,
    INSTANCES_DIR, VERSION_MANIFEST,
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
    pub game_metadata: GameVersionMetadata,
    pub mod_loader_version: Option<String>,
    pub mod_loader: ModLoader,
}

impl InstanceMetadata {
    fn new(
        name: &str,
        version: &str,
        mod_loader: ModLoader,
        mod_loader_version: Option<String>,
        icon: Option<String>,
    ) -> Result<Self, BackendError> {
        let version = VERSION_MANIFEST
            .versions()
            .find(|x| x.id == version)
            .ok_or(BackendError::InstanceError(
                InstanceError::MinecraftVersionNotFound(version.to_string()),
            ))?;

        std::fs::create_dir_all(INSTANCES_DIR.join(name))?;

        Ok(Self {
            name: name.to_string(),
            game_metadata: GameVersionMetadata {
                version: version.id.clone(),
                release_time: version.release_time.clone(),
                r#type: version.r#type.clone(),
            },
            icon,
            mod_loader,
            mod_loader_version: mod_loader_version,
        })
    }

    /// Creates a new instance, and adds it to the instances list at once
    pub fn create(
        name: &str,
        version: &str,
        mod_loader: ModLoader,
        mod_loader_version: Option<String>,
        icon: Option<String>,
    ) -> Result<Self, BackendError> {
        let instance = Self::new(name, version, mod_loader, mod_loader_version, icon)?;
        instances::add_new(&instance)?;
        Ok(instance)
    }

    async fn reinit_vanilla_client(
        &mut self,
        dir_path: &Path,
        client_json_path: &Path,
    ) -> Result<Client, BackendError> {
        dlog!("Re-initializing the instance");

        let client_raw = download_version(&self.game_metadata.version).await?;
        let client = serde_json::from_slice(&client_raw)?;

        tokio::fs::create_dir_all(dir_path).await?;
        tokio::fs::write(client_json_path, &client_raw).await?;
        Ok(client)
    }

    async fn reinit_mod_mod_loader(
        &self,
        mod_loader_json_path: &Path,
        java_path: &Path,
        javac_path: &Path,
    ) -> Result<Loaders, BackendError> {
        match self.mod_loader {
            ModLoader::Vanilla => Ok(Loaders::Vanilla),
            ModLoader::NeoForge => {
                install_neoforge_loader(self, java_path, javac_path, mod_loader_json_path)
                    .await
                    .map(|ok| Loaders::NeoForge(ok))
            }
            ModLoader::Fabric => install_fabric_loader(
                self,
                mod_loader_json_path,
                self.mod_loader_version.as_deref(),
            )
            .await
            .map(|ok| Loaders::Fabric(ok)),
            ModLoader::Quilt => install_quilt_loader(
                self,
                mod_loader_json_path,
                self.mod_loader_version.as_deref(),
            )
            .await
            .map(|ok| Loaders::Quilt(ok)),
            ModLoader::Forge => {
                install_forge_loader(self, java_path, javac_path, mod_loader_json_path)
                    .await
                    .map(|ok| Loaders::Forge(ok))
            }
        }
    }

    async fn init_vanilla_client(&mut self, dir_path: &Path) -> Result<Client, BackendError> {
        let client_json_path = dir_path.join("client.json");

        if !client_json_path.exists() {
            return self
                .reinit_vanilla_client(dir_path, &client_json_path)
                .await;
        }

        let client_json_file = std::fs::File::open(&client_json_path)?;
        let reader = std::io::BufReader::new(client_json_file);
        let client_json = serde_json::from_reader(reader)?;

        Ok(client_json)
    }

    async fn init_mod_loader(
        &mut self,
        dir_path: &Path,
        java_path: &Path,
        javac_path: &Path,
    ) -> Result<Loaders, BackendError> {
        if self.mod_loader == ModLoader::Vanilla {
            return Ok(Loaders::Vanilla);
        }

        let mod_loader_json_path = dir_path.join(format!("{}.json", self.mod_loader));
        if !mod_loader_json_path.exists() {
            return self
                .reinit_mod_mod_loader(&mod_loader_json_path, java_path, javac_path)
                .await;
        }

        let mod_loader_json = std::fs::File::open(mod_loader_json_path)?;
        let mod_loader: Loaders = serde_json::from_reader(mod_loader_json)?;
        Ok(mod_loader)
    }

    async fn load_config(
        &self,
        instance_dir: &Path,
        vanilla_client: &Client,
    ) -> Result<InstanceConfig, BackendError> {
        config::read_instance_config(instance_dir, &vanilla_client.java_version.component).await
    }

    fn instance_dir(&self) -> PathBuf {
        INSTANCES_DIR.join(&self.name)
    }

    fn minecraft_jar_path(&self) -> PathBuf {
        self.instance_dir()
            .join(format!("{}.jar", &self.game_metadata.version))
    }

    /// Loads ('Upgrades' information to) an instance's in memory representation
    pub async fn load_init(mut self) -> Result<LoadedInstance, BackendError> {
        let instance_dir = self.instance_dir();

        let vanilla_client = self.init_vanilla_client(&instance_dir).await?;
        let config = self.load_config(&instance_dir, &vanilla_client).await?;

        let java_path = config.java.java();
        let javac_path = &config.java.get_javac();

        let mod_loader = self
            .init_mod_loader(&instance_dir, java_path, javac_path)
            .await?;
        let client = mod_loader.concat(vanilla_client);

        let minecraft_jar_path = self.minecraft_jar_path();

        Ok(LoadedInstance {
            instance_metadata: self,
            config,
            client,
            minecraft_jar_path,
            instance_path: instance_dir,
        })
    }
}
