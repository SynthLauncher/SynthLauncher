use crate::{
    launcher::config::InstanceConfig,
    loaders::{
        fabric::install_fabric_loader, forge::install_forge_loader,
        neoforge::install_neoforge_loader, quilt::install_quilt_loader, Loaders,
    },
    minecraft::version_manifest::download_version_json,
    VERSIONS_DIR,
};

use super::instances::metadata::ModLoader;
use sl_meta::minecraft::loaders::vanilla::Client;
use sl_utils::{dlog, errors::BackendError};
use std::{
    fs::OpenOptions,
    io::BufReader,
    path::{Path, PathBuf},
};

pub(super) struct MinecraftVersionID {
    loader: ModLoader,
    loader_version: String,
    vanilla_version: String,
}

impl MinecraftVersionID {
    pub fn new(loader: ModLoader, loader_version: String, vanilla_version: String) -> Self {
        assert!(!loader_version.is_empty() || loader == ModLoader::Vanilla);
        Self {
            loader,
            loader_version,
            vanilla_version,
        }
    }

    fn dir_path(&self) -> PathBuf {
        VERSIONS_DIR.join(self.to_string())
    }

    fn client_jar_path(&self, dir_path: &Path) -> PathBuf {
        dir_path.join(format!("{}.jar", self.vanilla_version))
    }

    fn vanilla_json_path(&self, dir_path: &Path) -> PathBuf {
        dir_path.join("vanilla.json")
    }

    fn modloader_json_path(&self, dir_path: &Path) -> PathBuf {
        dir_path.join(format!("{}.json", self.loader))
    }

    pub fn to_string(&self) -> String {
        debug_assert!(!self.loader_version.is_empty() || self.loader == ModLoader::Vanilla);
        debug_assert!(!self.vanilla_version.is_empty());

        let loader_version = if self.loader_version.is_empty() {
            "none"
        } else {
            &self.loader_version.replace('-', "_")
        };

        format!(
            "{}-{}-{}",
            self.loader, loader_version, self.vanilla_version
        )
    }

    fn try_get_vanilla_json(&self, path: &Path) -> Option<Client> {
        let vanilla_json_file = std::fs::File::open(&path).ok()?;
        let reader = std::io::BufReader::new(vanilla_json_file);
        let vanilla_json = serde_json::from_reader(reader).ok()?;

        Some(vanilla_json)
    }

    async fn init_vanilla_json(&mut self, path: &Path) -> Result<Client, BackendError> {
        match self.try_get_vanilla_json(path) {
            Some(results) => Ok(results),
            None => self.reinit_vanilla_json(path).await,
        }
    }

    async fn load_config(
        &self,
        instance_dir: &Path,
        vanilla_client: &Client,
    ) -> Result<InstanceConfig, BackendError> {
        super::config::read_instance_config(instance_dir, &vanilla_client.java_version.component)
            .await
    }

    async fn reinit_modloader(
        &self,
        modloader_json_path: &Path,
        java_path: &Path,
        javac_path: &Path,
    ) -> Result<Loaders, BackendError> {
        assert!(!self.loader_version.is_empty() || self.loader == ModLoader::Vanilla);
        let loader_version = &self.loader_version;

        match self.loader {
            ModLoader::Vanilla => Ok(Loaders::Vanilla),
            ModLoader::NeoForge => install_neoforge_loader(
                &self.vanilla_version,
                loader_version,
                java_path,
                javac_path,
                modloader_json_path,
            )
            .await
            .map(|ok| Loaders::NeoForge(ok)),
            ModLoader::Fabric => {
                install_fabric_loader(&self.vanilla_version, modloader_json_path, loader_version)
                    .await
                    .map(|ok| Loaders::Fabric(ok))
            }
            ModLoader::Quilt => {
                install_quilt_loader(&self.vanilla_version, modloader_json_path, loader_version)
                    .await
                    .map(|ok| Loaders::Quilt(ok))
            }
            ModLoader::Forge => install_forge_loader(
                &self.vanilla_version,
                loader_version,
                java_path,
                javac_path,
                modloader_json_path,
            )
            .await
            .map(|ok| Loaders::Forge(ok)),
        }
    }

    fn try_get_modloader(&mut self, modloader_json_path: &Path) -> Option<Loaders> {
        let modloader_json_file = std::fs::File::open(modloader_json_path).ok()?;
        let reader = BufReader::new(modloader_json_file);
        serde_json::from_reader(reader).ok()
    }

    async fn init_mod_loader(
        &mut self,
        modloader_json_path: &Path,
        java_path: &Path,
        javac_path: &Path,
    ) -> Result<Loaders, BackendError> {
        if self.loader == ModLoader::Vanilla {
            return Ok(Loaders::Vanilla);
        }

        match self.try_get_modloader(modloader_json_path) {
            Some(results) => Ok(results),
            None => {
                self.reinit_modloader(&modloader_json_path, java_path, javac_path)
                    .await
            }
        }
    }

    async fn reinit_vanilla_json(
        &mut self,
        vanilla_json_path: &Path,
    ) -> Result<Client, BackendError> {
        dlog!("Re-initializing the instance");

        let vanilla_json_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(vanilla_json_path)?;
        download_version_json(&self.vanilla_version, vanilla_json_path).await?;

        let reader = std::io::BufReader::new(vanilla_json_file);
        let vanilla_json = serde_json::from_reader(reader)?;

        Ok(vanilla_json)
    }

    /// Loads, downloads and initializes a combination of a minecraft version, mod loader information, and an instance config
    /// also uses an instance directory to look for a config in before while performing this operation
    ///
    /// Config is required here for example for figuring out the path to java to use while installing forge/neoforge
    /// All of the files downloaded and initialized by this operation are returned in-memory
    pub async fn load_init(
        mut self,
        instance_dir: &Path,
    ) -> Result<(LoadedMinecraftVersion, InstanceConfig), BackendError> {
        let dir_path = self.dir_path();
        std::fs::create_dir_all(&dir_path)?;

        let client_jar_path = self.client_jar_path(&dir_path);
        let vanilla_json_path = self.vanilla_json_path(&dir_path);
        let modloader_json_path = self.modloader_json_path(&dir_path);

        let vanilla_client = self.init_vanilla_json(&vanilla_json_path).await?;
        let config = self.load_config(&instance_dir, &vanilla_client).await?;

        let java_path = config.java.java();
        let javac_path = &config.java.get_javac();

        let mod_loader = self
            .init_mod_loader(&modloader_json_path, java_path, javac_path)
            .await?;

        let minecraft_client_json = mod_loader.concat(vanilla_client);

        Ok((
            LoadedMinecraftVersion {
                client_jar_path,
                minecraft_client_json,
            },
            config,
        ))
    }
}

pub(super) struct LoadedMinecraftVersion {
    client_jar_path: PathBuf,
    minecraft_client_json: Client,
}

impl LoadedMinecraftVersion {
    pub const fn client_json(&self) -> &Client {
        &self.minecraft_client_json
    }

    pub fn client_jar_path(&self) -> &Path {
        self.client_jar_path.as_path()
    }
}
