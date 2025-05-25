use std::{
    borrow::Cow,
    ffi::OsStr,
    fmt,
    fs::{self, File},
    future::Future,
    io::BufReader,
    path::{Path, PathBuf},
    process::Stdio,
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sl_meta::json::{
    fabric::{self, profile::FabricLoaderProfile},
    quilt::profiles::{get_quilt_loader_profile, QuiltLoaderProfile},
    vanilla::Client,
    version_manifest::VersionType,
};
use sl_utils::utils::errors::{BackendError, DownloadError, InstallationError};
use tokio::process::Command;

use crate::{
    config::config::Config,
    json::{vanilla, version_manifest::download_version},
    profiles::player::PlayerProfile,
    ASSETS_DIR, INSTANCES_DIR, LIBS_DIR, MULTI_PATH_SEPARATOR, VERSION_MANIFEST,
};

#[derive(Debug, Deserialize)]
pub enum Loaders {
    Fabric(FabricLoaderProfile),
    Quilt(QuiltLoaderProfile),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum InstanceType {
    Vanilla,
    Fabric,
    Quilt, // We will add more
}

impl fmt::Display for InstanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            InstanceType::Vanilla => "vanilla",
            InstanceType::Fabric => "fabric",
            InstanceType::Quilt => "quilt",
        };
        write!(f, "{}", s)
    }
}

impl From<String> for InstanceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "vanilla" => InstanceType::Vanilla,
            "fabric" => InstanceType::Fabric,
            "quilt" => InstanceType::Quilt,
            _ => {
                panic!("Unknown instance type: {}", value)
            }
        }
    }
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
    pub icon: Option<PathBuf>,

    /*

    */
    pub instance_type: InstanceType,
}

impl Instance {
    pub fn new(
        name: &str,
        version: &str,
        instance_type: InstanceType,
        icon: Option<PathBuf>,
    ) -> Result<Self, BackendError> {
        let version = VERSION_MANIFEST
            .versions()
            .find(|x| x.id == version)
            .ok_or(BackendError::InstallationError(
                InstallationError::VersionNotFound(version.to_string()),
            ))?;

        Ok(Self {
            name: name.to_string(),
            game_info: InstanceGameInfo {
                version: version.id.clone(),
                release_time: version.release_time.clone(),
                r#type: version.r#type.clone(),
            },
            icon,
            instance_type,
        })
    }

    fn get_loader_from_dir(dir: &Path) -> Result<InstanceType, BackendError> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(file_name) = path.file_name().and_then(OsStr::to_str) {
                if file_name.contains("fabric.json") {
                    return Ok(InstanceType::Fabric);
                } else if file_name.contains("quilt.json") {
                    return Ok(InstanceType::Quilt);
                }
            }
        }

        Ok(InstanceType::Vanilla)
    }

    pub fn get_instance_from_dir(name: &str) -> Result<Self, BackendError> {
        let dir = Path::new(&INSTANCES_DIR.as_path()).join(&name);
        let path = dir.join("client.json");
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let client: Client = serde_json::from_reader(reader)?;

        let game_info = InstanceGameInfo {
            release_time: client.release_time,
            r#type: client.r#type,
            version: client.id,
        };

        let instance_type = Instance::get_loader_from_dir(&dir)?;

        Ok(Self {
            name: name.to_string(),
            game_info,
            icon: None,
            instance_type,
        })
    }

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
        self.dir_path().join("client.jar")
    }

    fn loader_json_path(&self) -> Option<PathBuf> {
        let path = format!("{}.json", self.instance_type);
        let path = self.dir_path().join(path);
        match self.instance_type {
            InstanceType::Fabric => Some(path),
            InstanceType::Quilt => Some(path),
            InstanceType::Vanilla => None,
        }
    }

    fn read_loader(&self) -> Option<Loaders> {
        match self.instance_type {
            InstanceType::Fabric => {
                let path = self.loader_json_path()?;

                let file = File::open(&path).ok()?;
                let profile: FabricLoaderProfile = serde_json::from_reader(file).ok()?;
                Some(Loaders::Fabric(profile))
            }
            InstanceType::Quilt => {
                let path = self.loader_json_path()?;
                let file = File::open(&path).ok()?;
                let profile: QuiltLoaderProfile = serde_json::from_reader(file).ok()?;
                Some(Loaders::Quilt(profile))
            }
            InstanceType::Vanilla => None,
        }
    }

    pub async fn install_loader(&mut self, loader_version: &str) -> Result<(), BackendError> {
        match self.instance_type {
            InstanceType::Vanilla => {
                return Ok(());
            }
            InstanceType::Fabric => {
                let path = self.dir_path().join("fabric.json");
                let make_req = async |url: &str| -> Result<Vec<u8>, DownloadError> {
                    let res = reqwest::get(url).await?;
                    let bytes = res.bytes().await?;
                    Ok(bytes.to_vec())
                };

                let profile = fabric::profile::get_loader_profile::<
                    fn(&str) -> dyn Future<Output = Result<Vec<u8>, DownloadError>>,
                    DownloadError,
                >(&self.game_info.version, loader_version, make_req)
                .await?;
                let file = File::create(&path)?;

                serde_json::to_writer_pretty(file, &profile)?;

                Ok(())
            }
            InstanceType::Quilt => {
                let path = self.dir_path().join("quilt.json");
                let make_req = async |url: &str| -> Result<Vec<u8>, DownloadError> {
                    let res = reqwest::get(url).await?;
                    let bytes = res.bytes().await?;
                    Ok(bytes.to_vec())
                };

                let profile = get_quilt_loader_profile::<
                    fn(&str) -> dyn Future<Output = Result<Vec<u8>, DownloadError>>,
                    DownloadError,
                >(&self.game_info.version, loader_version, make_req)
                .await?;
                let file = File::create(&path)?;

                serde_json::to_writer_pretty(file, &profile)?;

                Ok(())
            }
        }
    }

    async fn read_client_raw(&self) -> Option<Client> {
        let client = tokio::fs::read_to_string(&self.client_json_path())
            .await
            .ok()?;
        Some(serde_json::from_str(&client).expect("Failed to deserialize client.json!"))
    }

    async fn read_client(&self) -> Option<Client> {
        let mut client = self.read_client_raw().await?;

        if let Some(loader) = self.read_loader() {
            match loader {
                Loaders::Fabric(fabric) => {
                    client = fabric.join_client(client);
                    return Some(client);
                }
                Loaders::Quilt(quilt) => {
                    client = quilt.join_client(client);
                    return Some(client);
                }
            }
        }

        Some(client)
    }

    async fn read_config(&self) -> Option<Config> {
        let config = tokio::fs::read_to_string(self.config_path()).await.ok()?;

        Some(serde_json::from_str(&config).expect("Failed to deserialize config.json!"))
    }

    async fn override_config(&mut self, config: Config) -> Result<(), std::io::Error> {
        let installation_dir = self.dir_path();
        let config_path = self.config_path();

        tokio::fs::create_dir_all(&installation_dir).await?;
        tokio::fs::write(&config_path, serde_json::to_string_pretty(&config)?).await?;
        Ok(())
    }

    async fn reinit(&mut self) -> Result<Client, BackendError> {
        let client_raw = download_version(&self.game_info.version).await?;
        let client: Client =
            serde_json::from_slice(&client_raw).expect("Failed to deserialize client.json!");

        let config = Config::create_config(&client.java_version.component).await?;
        self.override_config(config).await?;

        tokio::fs::create_dir_all(self.dir_path()).await?;
        tokio::fs::write(self.client_json_path(), &client_raw).await?;
        Ok(client)
    }

    pub async fn init(&mut self) -> Result<Client, BackendError> {
        match self.read_client().await {
            Some(client) => Ok(client),
            None => self.reinit().await,
        }
    }

    pub async fn install(&mut self) -> Result<(), BackendError> {
        let client = self.init().await?;

        vanilla::install_client(&client, &self.dir_path()).await
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
        config: &Config,
        profile: Option<&PlayerProfile>,
    ) -> Result<Vec<String>, BackendError> {
        let global_config = Config::read_global().unwrap();
        let client = self
            .read_client()
            .await
            .expect("Failed to read client.json!");
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
                "auth_uuid" => profile.map(|m| m.uuid.as_str()).unwrap_or("0"),
                "auth_access_token" => profile.map(|m| m.access_token.as_str()).unwrap_or("0"),
                "auth_player_name" => profile
                    .map(|m| m.username.as_str())
                    .unwrap_or(global_config.get("auth_player_name").unwrap()),
                "clientid" => "74909cec-49b6-4fee-aa60-1b2a57ef72e1", // Please don't steal :(
                "version_type" => "SynthLauncher",
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

    pub async fn execute(&self, profile: Option<&PlayerProfile>) -> Result<(), BackendError> {
        let config = self.read_config().await.unwrap();
        let current_java_path = config.get("java").unwrap();

        let max_ram = config.get("max_ram").unwrap_or("2048");
        let min_ram = config.get("min_ram").unwrap_or("1024");

        let args = self.generate_arguments(&config, profile).await?;

        println!("Launching with args: {:?}", &args);

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
            return Err(BackendError::InstallationError(
                InstallationError::FailedToExecute(self.name.clone()),
            ));
        }

        Ok(())
    }
}
