use std::{
    borrow::Cow,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
    process::Stdio,
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sl_meta::minecraft::{loaders::vanilla::Client, version_manifest::VersionType};
use sl_utils::{
    dlog, elog, log,
    utils::errors::{BackendError, InstanceError},
    wlog,
};
use strum_macros::{AsRefStr, Display, EnumString};
use tokio::process::Command;

use crate::{
    launcher::{
        config::InstanceConfig,
        instances::{self},
        player::{player_profile::PlayerProfile, player_profiles::PlayerProfiles},
    },
    loaders::{
        fabric::install_fabric_loader, forge::install_forge_loader,
        neoforge::install_neoforge_loader, quilt::install_quilt_loader, Loaders,
    },
    minecraft::{install_client, version_manifest::download_version},
    ASSETS_DIR, INSTANCES_DIR, LIBS_DIR, MULTI_PATH_SEPARATOR, VERSION_MANIFEST,
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
pub enum InstanceType {
    #[default]
    Vanilla,
    Fabric,
    Quilt,
    Forge,
    NeoForge,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstanceGameInfo {
    #[serde(rename = "id")]
    pub version: String,
    pub release_time: String,
    pub r#type: VersionType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstanceInfo {
    /// The name of the instance
    pub name: String,
    pub game_info: InstanceGameInfo,
    /// Base 64 Image
    pub icon: Option<String>,
    pub modloader_version: Option<String>,
    pub instance_type: InstanceType,
}

impl InstanceInfo {
    fn new(
        name: &str,
        version: &str,
        instance_type: InstanceType,
        loader_version: Option<String>,
        icon: Option<String>,
    ) -> Result<Self, BackendError> {
        let version = VERSION_MANIFEST
            .versions()
            .find(|x| x.id == version)
            .ok_or(BackendError::InstanceError(InstanceError::VersionNotFound(
                version.to_string(),
            )))?;

        fs::create_dir_all(INSTANCES_DIR.join(name))?;

        Ok(Self {
            name: name.to_string(),
            game_info: InstanceGameInfo {
                version: version.id.clone(),
                release_time: version.release_time.clone(),
                r#type: version.r#type.clone(),
            },
            icon,
            instance_type,
            modloader_version: loader_version,
        })
    }

    /// Creates a new instance, and adds it to the instances list at once
    pub fn create(
        name: &str,
        version: &str,
        instance_type: InstanceType,
        loader_version: Option<String>,
        icon: Option<String>,
    ) -> Result<Self, BackendError> {
        let instance = Self::new(name, version, instance_type, loader_version, icon)?;
        instances::add_new(&instance)?;
        Ok(instance)
    }

    async fn reinit_vanilla_client(
        &mut self,
        dir_path: &Path,
        client_json_path: &Path,
    ) -> Result<Client, BackendError> {
        dlog!("Re-initializing the instance");

        let client_raw = download_version(&self.game_info.version).await?;
        let client = serde_json::from_slice(&client_raw)?;

        tokio::fs::create_dir_all(dir_path).await?;
        tokio::fs::write(client_json_path, &client_raw).await?;
        Ok(client)
    }

    async fn reinit_mod_loader(
        &self,
        loader_json_path: &Path,
        java_path: &Path,
        javac_path: &Path,
    ) -> Result<Loaders, BackendError> {
        match self.instance_type {
            InstanceType::Vanilla => Ok(Loaders::Vanilla),
            InstanceType::NeoForge => {
                install_neoforge_loader(self, java_path, javac_path, loader_json_path)
                    .await
                    .map(|ok| Loaders::NeoForge(ok))
            }
            InstanceType::Fabric => {
                install_fabric_loader(self, loader_json_path, self.modloader_version.as_deref())
                    .await
                    .map(|ok| Loaders::Fabric(ok))
            }
            InstanceType::Quilt => {
                install_quilt_loader(self, loader_json_path, self.modloader_version.as_deref())
                    .await
                    .map(|ok| Loaders::Quilt(ok))
            }
            InstanceType::Forge => {
                install_forge_loader(self, java_path, javac_path, loader_json_path)
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

        let client_json_file = File::open(&client_json_path)?;
        let reader = BufReader::new(client_json_file);
        let client_json = serde_json::from_reader(reader)?;

        Ok(client_json)
    }

    async fn init_loader(
        &mut self,
        dir_path: &Path,
        java_path: &Path,
        javac_path: &Path,
    ) -> Result<Loaders, BackendError> {
        if self.instance_type == InstanceType::Vanilla {
            return Ok(Loaders::Vanilla);
        }

        let loader_json_path = dir_path.join(format!("{}.json", self.instance_type));
        if !loader_json_path.exists() {
            return self
                .reinit_mod_loader(&loader_json_path, java_path, javac_path)
                .await;
        }

        let loader_json = File::open(loader_json_path)?;
        let loader: Loaders = serde_json::from_reader(loader_json)?;
        Ok(loader)
    }

    async fn load_config(
        &self,
        instance_dir: &Path,
        vanilla_client: &Client,
    ) -> Result<InstanceConfig, BackendError> {
        super::config::read_instance_config(instance_dir, &vanilla_client.java_version.component)
            .await
    }

    fn instance_dir(&self) -> PathBuf {
        INSTANCES_DIR.join(&self.name)
    }

    fn minecraft_jar_path(&self) -> PathBuf {
        self.instance_dir()
            .join(format!("{}.jar", &self.game_info.version))
    }

    /// Loads ('Upgrades' information to) an instance's in memory representation
    pub async fn load_init(mut self) -> Result<LoadedInstance, BackendError> {
        let instance_dir = self.instance_dir();

        let vanilla_client = self.init_vanilla_client(&instance_dir).await?;
        let config = self.load_config(&instance_dir, &vanilla_client).await?;

        let java_path = config.java.java();
        let javac_path = &config.java.get_javac();

        let loader = self
            .init_loader(&instance_dir, java_path, javac_path)
            .await?;
        let client = loader.concat(vanilla_client);

        let minecraft_jar_path = self.minecraft_jar_path();

        Ok(LoadedInstance {
            info: self,
            config,
            client,
            minecraft_jar_path,
            instance_path: instance_dir,
        })
    }
}

/// Represents a loaded instance of Minecraft with its configurations and things required for launching
pub struct LoadedInstance {
    info: InstanceInfo,

    config: InstanceConfig,
    client: Client,

    minecraft_jar_path: PathBuf,
    instance_path: PathBuf,
}

impl LoadedInstance {
    const fn game_info(&self) -> &InstanceGameInfo {
        &self.info.game_info
    }

    fn instance_dir(&self) -> &Path {
        &self.instance_path
    }

    /// Downloads the required files for executing minecraft for this instance, only downloads corrupted or non existing files
    ///
    /// the reason why the download operation is not a part of the init, is because this operation results is not part of the instance's memory representation
    /// so i think splitting them makes the code more maintainable
    async fn download_minecraft(&self) -> Result<(), BackendError> {
        // will automatically perform hash verification and only re install corrupted files
        install_client(&self.client, &self.minecraft_jar_path, &self.instance_path).await?;
        Ok(())
    }

    /// Generates the classpath for executing minecraft for this instance
    fn generate_classpath(&self) -> String {
        let libs = self.client.libraries();

        let mut classpath = Vec::new();
        for lib in libs {
            if let Some(ref native) = lib.native_from_platform() {
                let path = native.path.as_ref().unwrap();
                let full_path = LIBS_DIR.join(path);
                classpath.push(format!("{}", full_path.display()));
            }
            if let Some(ref artifact) = lib.downloads.artifact {
                let path = artifact.path.as_ref().unwrap();
                let full_path = LIBS_DIR.join(path);
                classpath.push(format!("{}", full_path.display()));
            }
        }

        let minecraft_jar = &self.minecraft_jar_path;
        classpath.push(minecraft_jar.to_string_lossy().into_owned());
        classpath.join(MULTI_PATH_SEPARATOR)
    }

    // Thanks MrMayMan
    fn generate_sound_arguments(&self, jvm_args: &mut Vec<String>) {
        if self.game_info().r#type == VersionType::OldBeta
            || self.game_info().r#type == VersionType::OldAlpha
        {
            jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());

            if self.game_info().version.starts_with("c0.") {
                // Classic
                jvm_args.push("-Dhttp.proxyPort=11701".to_owned());
            } else if self.game_info().r#type == VersionType::OldAlpha {
                // Indev, Infdev and Alpha (mostly same)
                jvm_args.push("-Dhttp.proxyPort=11702".to_owned());
            } else {
                // Beta
                jvm_args.push("-Dhttp.proxyPort=11705".to_owned());
            }

            // Fixes crash on old versions
            jvm_args.push("-Djava.util.Arrays.useLegacyMergeSort=true".to_owned());
        } else {
            // 1.5.2 release date
            let v1_5_2 = DateTime::parse_from_rfc3339("2013-04-25T15:45:00+00:00").unwrap();
            let release = DateTime::parse_from_rfc3339(&self.game_info().release_time).unwrap();

            if release <= v1_5_2 {
                // 1.0 - 1.5.2
                jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());
                jvm_args.push("-Dhttp.proxyPort=11707".to_owned());
            }
        }
    }

    async fn generate_arguments(
        &self,
        profile: &PlayerProfile,
    ) -> Result<Vec<String>, BackendError> {
        let classpath = self.generate_classpath();
        let game_dir = self.instance_dir();

        let natives_dir = game_dir.join(".natives");

        let raw_args = &self.client.arguments;
        let (mut jvm_args, mut game_args) = raw_args.clone().into_raw();

        let regex = regex::Regex::new(r"\$\{(\w+)\}").expect("Failed to compile regex!");

        self.generate_sound_arguments(&mut jvm_args);

        let fmt_arg = |arg: &str| {
            Some(match arg {
                "game_directory" => game_dir.to_str().unwrap(),
                "assets_root" | "game_assets" => ASSETS_DIR.to_str().unwrap(),
                "assets_index_name" => &self.client.assets,
                "version_name" => &self.game_info().version,
                "classpath" => classpath.as_str(),
                "natives_directory" => natives_dir.to_str().unwrap(),
                "auth_uuid" => &profile.data.uuid,
                "auth_access_token" => &profile.access_token,
                "auth_player_name" => &profile.data.username,
                "clientid" => "74909cec-49b6-4fee-aa60-1b2a57ef72e1", // Please don't steal :(
                "version_type" => "SL",
                "library_directory" => LIBS_DIR.to_str().unwrap(),
                "classpath_separator" => MULTI_PATH_SEPARATOR,
                other => {
                    wlog!(
                        "Couldn't evaluate argument: {}, for launching minecraft",
                        other
                    );
                    return None;
                }
            })
        };

        let fmt_args = |args: &mut Vec<String>| {
            for arg in args {
                let new_value = regex.replace_all(&arg, |caps: &regex::Captures| {
                    let fmt_spec = caps.get(1).unwrap().as_str();
                    fmt_arg(fmt_spec).unwrap_or_default()
                });

                if let Cow::Owned(value) = new_value {
                    *arg = value;
                }
            }
        };

        fmt_args(&mut game_args);
        fmt_args(&mut jvm_args);

        jvm_args.push(self.client.main_class.clone());

        Ok([jvm_args, game_args].concat())
    }

    /// Performs the execution of the instance.
    pub async fn execute(self) -> Result<(), BackendError> {
        // the reason why the download operation is done here is to ensure that the files are available before executing the instance.
        // AND THE REASON WHY YOU DON'T LEAVE CALLING THIS TO THE CALLER OF THE EXECUTE METHOD is because it is just better and cleaner,
        // you should aim to ensure that the caller will get a compile time error instead of causing a runtime bug and each exported function should be self-contained.
        self.download_minecraft().await?;

        let profiles = PlayerProfiles::load()?;
        let profile = profiles.current_profile();

        log!(
            "Executing instance '{}' with type '{:?}', using profile '{}'",
            self.info.name,
            self.info.instance_type,
            profile.data.username
        );

        let current_java_path = self.config.java.java();

        let max_ram = self.config.java.max_ram;
        let min_ram = self.config.java.min_ram;

        let args = self.generate_arguments(&profile).await?;

        dlog!("Launching with args: {:?}", &args);

        let output = Command::new(current_java_path)
            .arg(format!("-Xmx{}M", max_ram))
            .arg(format!("-Xms{}M", min_ram))
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            elog!("stderr:\n{}\nstdout:\n{}", stderr, stdout);
            return Err(BackendError::InstanceError(InstanceError::FailedToExecute(
                self.info.name.clone(),
            )));
        }

        Ok(())
    }
}
