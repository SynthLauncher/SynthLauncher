use std::{
    borrow::Cow,
    fs::{self},
    path::PathBuf,
    process::Stdio,
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sl_meta::minecraft::{
    loaders::{
        fabric::profile::FabricLoaderProfile, forge::ForgeLoaderProfile,
        neoforge::NeoForgeLoaderProfile, quilt::profiles::QuiltLoaderProfile, vanilla::Client,
    },
    version_manifest::VersionType,
};
use sl_utils::{
    dlog, elog, log,
    utils::errors::{BackendError, InstanceError},
};
use strum_macros::{AsRefStr, Display, EnumString};
use tokio::process::Command;

use crate::{
    launcher::{config::Config, instances::Instances, player::player_profile::PlayerProfile},
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
pub struct Instance {
    pub name: String,
    pub game_info: InstanceGameInfo,

    /*
        If none instance tries to get it from
        INSTANCES_PATH/instance_name/icon.png
    */
    // TODO: Change this to a string
    pub icon: Option<PathBuf>,
    pub modloader_version: Option<String>,
    pub instance_type: InstanceType,
}

impl Instance {
    /// Creates a new instance, and adds it to the instances list at once
    pub fn create(
        name: &str,
        version: &str,
        instance_type: InstanceType,
        loader_version: Option<String>,
        icon: Option<PathBuf>,
    ) -> Result<Self, BackendError> {
        let instance = Self::new(name, version, instance_type, loader_version, icon)?;
        Instances::add(&instance)?;
        Ok(instance)
    }

    fn new(
        name: &str,
        version: &str,
        instance_type: InstanceType,
        loader_version: Option<String>,
        icon: Option<PathBuf>,
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

    // TODO: Change how this works
    // fn get_loader_from_dir(dir: &Path) -> Result<InstanceType, BackendError> {
    //     for entry in fs::read_dir(dir)? {
    //         let entry = entry?;
    //         let path = entry.path();

    //         if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
    //             if file_name.contains("fabric.json") {
    //                 return Ok(InstanceType::Fabric);
    //             } else if file_name.contains("quilt.json") {
    //                 return Ok(InstanceType::Quilt);
    //             }
    //         }
    //     }

    //     Ok(InstanceType::Vanilla)
    // }

    // // TODO: Change how this works
    // pub fn get_instance_from_dir(name: &str) -> Result<Self, BackendError> {
    //     let dir = Path::new(&INSTANCES_DIR.as_path()).join(&name);
    //     let path = dir.join("client.json");
    //     let file = std::fs::File::open(path)?;
    //     let reader = BufReader::new(file);
    //     let client: Client = serde_json::from_reader(reader)?;

    //     let game_info = InstanceGameInfo {
    //         release_time: client.release_time,
    //         r#type: client.r#type,
    //         version: client.id,
    //     };

    //     let instance_type = Instance::get_loader_from_dir(&dir)?;

    //     Ok(Self {
    //         name: name.to_string(),
    //         game_info,
    //         icon: None,
    //         instance_type,
    //     })
    // }

    pub fn dir_path(&self) -> PathBuf {
        INSTANCES_DIR.join(&self.name)
    }

    fn config_path(&self) -> PathBuf {
        self.dir_path().join("config.json")
    }

    fn client_json_path(&self) -> PathBuf {
        self.dir_path().join("client.json")
    }

    fn client_jar_path(&self) -> PathBuf {
        self.dir_path()
            .join(format!("{}.jar", &self.game_info.version))
    }

    pub fn loader_json_path(&self) -> Option<PathBuf> {
        let path = format!("{}.json", self.instance_type);
        let path = self.dir_path().join(path);
        match self.instance_type {
            InstanceType::Fabric
            | InstanceType::Forge
            | InstanceType::Quilt
            | InstanceType::NeoForge => Some(path),
            InstanceType::Vanilla => None,
        }
    }

    /// Reads the loader information from the JSON file, returns Some if the loader exists and is correctly initialized otherwise None
    fn read_loader_init(&self) -> Option<Loaders> {
        macro_rules! generic_concat_loader {
            ($variant: ident, $profile_type: ty) => {{
                let path = self.loader_json_path()?;
                let file = fs::File::open(&path).ok()?;
                let profile: $profile_type = serde_json::from_reader(file).ok()?;
                println!("Returning variant {}", stringify!($variant));
                Loaders::$variant(profile)
            }};
        }

        Some(match self.instance_type {
            InstanceType::Fabric => generic_concat_loader!(Fabric, FabricLoaderProfile),
            InstanceType::Quilt => generic_concat_loader!(Quilt, QuiltLoaderProfile),
            InstanceType::Forge => generic_concat_loader!(Forge, ForgeLoaderProfile),
            InstanceType::NeoForge => generic_concat_loader!(NeoForge, NeoForgeLoaderProfile),
            InstanceType::Vanilla => Loaders::Vanilla,
        })
    }

    async fn reinit_loader(&self) -> Result<Loaders, BackendError> {
        let loader_version = self.modloader_version.as_deref();

        match self.instance_type {
            InstanceType::Vanilla => {
                return Ok(Loaders::Vanilla);
            }
            InstanceType::Fabric => Ok(Loaders::Fabric(
                install_fabric_loader(&self, loader_version).await?,
            )),
            InstanceType::Quilt => Ok(Loaders::Quilt(
                install_quilt_loader(&self, loader_version).await?,
            )),
            InstanceType::NeoForge => Ok(Loaders::NeoForge(install_neoforge_loader(self).await?)),
            InstanceType::Forge => Ok(Loaders::Forge(install_forge_loader(self).await?)),
        }
    }

    /// Reads the vanilla client.json file and returns the client information, returns None if the file does not exist or deserialization fails.
    async fn read_vanilla_client(&self) -> Option<Client> {
        let client = tokio::fs::read_to_string(&self.client_json_path())
            .await
            .ok()?;
        // instead of returning an error, return None if deserialization fails so that it is deserialized
        serde_json::from_str(&client).ok()
    }

    fn read_config(&self) -> Option<Config> {
        let file = fs::File::open(self.config_path()).ok()?;
        Some(serde_json::from_reader(file).expect("Failed to deserialize config.json!"))
    }

    async fn override_config(&mut self, config: Config) -> Result<(), std::io::Error> {
        let installation_dir = self.dir_path();
        let config_path = self.config_path();

        tokio::fs::create_dir_all(&installation_dir).await?;
        let file = fs::File::create(config_path)?;
        serde_json::to_writer_pretty(file, &config)?;
        Ok(())
    }

    async fn reinit(&mut self) -> Result<Client, BackendError> {
        dlog!("Re-initializing the instance");

        let client_raw = download_version(&self.game_info.version).await?;
        let client: Client =
            serde_json::from_slice(&client_raw).expect("Failed to deserialize client.json!");

        // FIXME: this should only re-initialize the client and not the config, basing the existence of a config on the existence of a client is not a good idea,
        // however the config needs the client's java version to be initialized so i couldn't figure out how to do it without creating a config
        let config = Config::create_local_config(&client.java_version.component).await?;
        self.override_config(config).await?;

        tokio::fs::create_dir_all(self.dir_path()).await?;
        tokio::fs::write(self.client_json_path(), &client_raw).await?;
        Ok(client)
    }

    /// Initializes the mod-loader lazily, doesn't do anything if the loader is already initialized, re initializes if corrupted.
    #[must_use]
    async fn init_loader(&mut self) -> Result<Loaders, BackendError> {
        match self.instance_type {
            InstanceType::Vanilla => Ok(Loaders::Vanilla),
            InstanceType::Forge
            | InstanceType::Fabric
            | InstanceType::Quilt
            | InstanceType::NeoForge => {
                if let Some(loader) = self.read_loader_init() {
                    Ok(loader)
                } else {
                    // the loader is not initialized or corrupted, re-initialize it
                    self.reinit_loader().await
                }
            }
        }
    }

    /// Initializes the vanilla client lazily, doesn't do anything if already initialized, re initializes if corrupted.
    #[must_use]
    async fn init_vanilla_client(&mut self) -> Result<Client, BackendError> {
        match self.read_vanilla_client().await {
            Some(client) => Ok(client),
            None => self.reinit().await,
        }
    }

    // PLEASE S I BEG YOU DON'T CHANGE THIS....
    /// Initializes the instance lazily, doesn't do anything if already initialized.
    async fn init(&mut self) -> Result<Client, BackendError> {
        let vanilla_client = self.init_vanilla_client().await?;
        let loader = self.init_loader().await?;
        let client = loader.concat(vanilla_client);
        // will automatically perform hash verification and only re install corrupted files
        install_client(&client, self.client_jar_path(), &self.dir_path()).await?;
        Ok(client)
    }

    fn classpath(&self, client: &Client) -> String {
        let libs = client.libraries();

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

        let client_jar = self.client_jar_path();
        classpath.push(format!("{}", client_jar.display()));
        classpath.join(MULTI_PATH_SEPARATOR)
    }

    // Thanks MrMayMan
    fn generate_sound_arguments(&self, jvm_args: &mut Vec<String>) {
        if self.game_info.r#type == VersionType::OldBeta
            || self.game_info.r#type == VersionType::OldAlpha
        {
            jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());

            if self.game_info.version.starts_with("c0.") {
                // Classic
                jvm_args.push("-Dhttp.proxyPort=11701".to_owned());
            } else if self.game_info.r#type == VersionType::OldAlpha {
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
            let release = DateTime::parse_from_rfc3339(&self.game_info.release_time).unwrap();

            if release <= v1_5_2 {
                // 1.0 - 1.5.2
                jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());
                jvm_args.push("-Dhttp.proxyPort=11707".to_owned());
            }
        }
    }

    async fn generate_arguments(
        &self,
        client: Client,
        config: &Config,
        profile: &PlayerProfile,
    ) -> Result<Vec<String>, BackendError> {
        let classpath = self.classpath(&client);
        let game_dir = self.dir_path();
        let natives_dir = game_dir.join(".natives");

        let raw_args = client.arguments;
        let (mut jvm_args, mut game_args) = raw_args.into_raw();

        let regex = regex::Regex::new(r"\$\{(\w+)\}").expect("Failed to compile regex!");

        self.generate_sound_arguments(&mut jvm_args);

        let fmt_arg = |arg: &str| {
            Some(match arg {
                "game_directory" => game_dir.to_str().unwrap(),
                "assets_root" | "game_assets" => ASSETS_DIR.to_str().unwrap(),
                "assets_index_name" => &client.assets,
                "version_name" => &self.game_info.version,
                "classpath" => classpath.as_str(),
                "natives_directory" => natives_dir.to_str().unwrap(),
                "auth_uuid" => &profile.data.uuid,
                "auth_access_token" => &profile.access_token,
                "auth_player_name" => &profile.data.username,
                "clientid" => "74909cec-49b6-4fee-aa60-1b2a57ef72e1", // Please don't steal :(
                "version_type" => "SL",
                "library_directory" => LIBS_DIR.to_str().unwrap(),
                "classpath_separator" => MULTI_PATH_SEPARATOR,
                _ => config.get(arg)?,
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

        jvm_args.push(client.main_class.clone());

        Ok([jvm_args, game_args].concat())
    }

    pub fn get_java(&self) -> PathBuf {
        // FIXME: there should be a java to default to in the global config
        // and we shouldn't assume that there is an existing local config
        // instead we should have a function that always returns a config not an Option<Config>, returns the global one if there is no local, combines with the global if there is
        // TODO: cache the Config or at least the java path in memory using Once?
        let config = self.read_config().unwrap();
        let results = PathBuf::from(config.get("java").unwrap());
        debug_assert!(results.exists());
        results
    }

    /// Returns the path to the javac executable which the current instance uses
    /// gruannted to exist otherwise it panciks in debug mode
    pub fn get_javac(&self) -> PathBuf {
        // FIXME: there should be a better implementition
        let java = self.get_java();
        let ext = java.extension();
        let mut results = java.with_file_name("javac");
        if let Some(ext) = ext {
            results.set_extension(ext);
        }
        debug_assert!(results.exists());
        results
    }

    pub async fn execute(&mut self, profile: &PlayerProfile) -> Result<(), BackendError> {
        let client = self.init().await?;

        log!(
            "Executing instance '{}' with type '{:?}', using profile '{}'",
            self.name,
            self.instance_type,
            profile.data.username
        );

        let config = self.read_config().unwrap();
        let current_java_path = config.get("java").unwrap();

        let max_ram = config.get("max_ram").unwrap_or("2048");
        let min_ram = config.get("min_ram").unwrap_or("1024");

        let args = self.generate_arguments(client, &config, profile).await?;

        // !!! Warning if you're recording your auth_token may get leaked XD
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
                self.name.clone(),
            )));
        }

        Ok(())
    }
}
