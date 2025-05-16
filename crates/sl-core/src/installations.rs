use std::{
    borrow::Cow,
    fs::{self, File, OpenOptions},
    future::Future,
    io::{self, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sl_meta::json::{
    fabric::{self, profile::FabricLoaderProfile},
    vanilla::Client,
    version_manifest::VersionType,
};
use sl_utils::utils::errors::{BackendError, DownloadError, InstallationError};

use crate::{
    auth::PlayerProfile,
    config::config::Config,
    json::{client, manifest::download_version},
    ASSETS_DIR, INSTALLATIONS_DIR, INSTALLATIONS_PATH, LIBS_DIR, MANIFEST, MULTI_PATH_SEPARATOR,
    TEMP_CLIENT,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstallationInfo {
    #[serde(rename = "id")]
    pub version: String,
    pub release_time: String,
    pub r#type: Option<VersionType>, // TODO: Add icon
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Installation {
    pub name: String,
    pub info: InstallationInfo,
}

impl Installation {
    pub fn new(name: &str, version: &str) -> Option<Self> {
        MANIFEST
            .versions()
            .find(|x| x.id == version)
            .and_then(|version| {
                Some(Self {
                    name: name.to_owned(),
                    info: InstallationInfo {
                        version: version.id.clone(),
                        release_time: version.release_time.clone(),
                        r#type: Some(version.r#type),
                    },
                })
            })
    }

    pub fn get_installation_from_dir(name: &str) -> Result<Self, BackendError> {
        let path = Path::new(&INSTALLATIONS_DIR.as_path())
            .join(&name)
            .join("client.json");
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let client: Client = serde_json::from_reader(reader)?;

        let info = InstallationInfo {
            release_time: client.release_time,
            r#type: Some(client.r#type),
            version: client.id,
        };

        Ok(Self {
            name: name.to_string(),
            info,
        })
    }

    pub fn dir_path(&self) -> PathBuf {
        INSTALLATIONS_DIR.join(&self.name)
    }

    fn config_path(&self) -> PathBuf {
        self.dir_path().join("config.json")
    }

    fn client_json_path(&self) -> PathBuf {
        self.dir_path().join("client.json")
    }

    fn fabric_json_path(&self) -> Option<PathBuf> {
        let path = self.dir_path().join("fabric.json");
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    fn read_fabric(&self) -> Option<FabricLoaderProfile> {
        let path = self.fabric_json_path()?;
        let file = File::open(&path).ok()?;
        let profile = serde_json::from_reader(file).ok()?;
        Some(profile)
    }

    pub async fn install_fabric(&mut self, loader_version: &str) -> Result<(), BackendError> {
        if self.fabric_json_path().is_some() {
            return Ok(());
        }

        let path = self.dir_path().join("fabric.json");

        let make_request = async |url: &str| -> Result<Vec<u8>, DownloadError> {
            let response = reqwest::get(url).await?;
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        };

        let profile = fabric::profile::get_loader_profile::<
            fn(&str) -> dyn Future<Output = Result<Vec<u8>, DownloadError>>,
            DownloadError,
        >(&self.info.version, loader_version, make_request)
        .await?;
        let file = File::create(&path)?;

        serde_json::to_writer_pretty(file, &profile)?;

        Ok(())
    }

    fn client_jar_path(&self) -> PathBuf {
        self.dir_path().join("client.jar")
    }

    fn read_config(&self) -> Option<Config> {
        let config = fs::read_to_string(self.config_path()).ok()?;

        Some(serde_json::from_str(&config).expect("Failed to deserialize config.json!"))
    }

    fn read_client_raw(&self) -> Option<Client> {
        let client = fs::read_to_string(self.client_json_path()).ok()?;
        Some(serde_json::from_str(&client).expect("Failed to deserialize client.json!"))
    }

    fn read_client(&self) -> Option<Client> {
        let mut client = self.read_client_raw()?;
        if let Some(fabric) = self.read_fabric() {
            client = fabric.join_client(client);
        }
        Some(client)
    }

    fn override_config(&mut self, config: Config) -> Result<(), std::io::Error> {
        let installations_dir = self.dir_path();
        let config_path = self.config_path();

        fs::create_dir_all(&installations_dir)?;
        fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
        Ok(())
    }

    async fn reinit(&mut self) -> Result<Client, BackendError> {
        let client_raw = download_version(&self.info.version).await?;
        let client: Client =
            serde_json::from_slice(&client_raw).expect("Failed to deserialize client.json!");

        let config =
            Config::create_config(client.java_version.as_ref().unwrap().major_version).await?;
        let config = config.merge(Config::read_global().unwrap());
        self.override_config(config)?;

        fs::create_dir_all(self.dir_path())?;

        fs::write(self.client_json_path(), &client_raw)?;
        Ok(client)
    }

    pub async fn init(&mut self) -> Result<Client, BackendError> {
        match self.read_client() {
            Some(client) => Ok(client),
            None => self.reinit().await,
        }
    }

    pub async fn install(&mut self) -> Result<(), BackendError> {
        let client = self.init().await?;
        *TEMP_CLIENT.lock().await = Some(client);

        client::install_client(self.dir_path()).await
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
        if self.info.r#type == Some(VersionType::OldBeta)
            || self.info.r#type == Some(VersionType::OldAlpha)
        {
            jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());

            if self.info.version.starts_with("c0.") {
                // Classic
                jvm_args.push("-Dhttp.proxyPort=11701".to_owned());
            } else if self.info.r#type == Some(VersionType::OldAlpha) {
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
            let release = DateTime::parse_from_rfc3339(&self.info.release_time).unwrap();

            if release <= v1_5_2 {
                // 1.0 - 1.5.2
                jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());
                jvm_args.push("-Dhttp.proxyPort=11707".to_owned());
            }
        }
    }

    fn generate_arguments(
        &self,
        config: &Config,
        profile: Option<&PlayerProfile>,
    ) -> Result<Vec<String>, BackendError> {
        let global_config = Config::read_global().unwrap();
        let client = self.read_client().expect("Failed to read client.json!");
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
                "version_name" => &self.info.version,
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

    pub fn execute(&self, profile: Option<&PlayerProfile>) -> Result<(), BackendError> {
        let config = self.read_config().unwrap();

        let current_java_path = config.get("java").unwrap();

        println!("Trying to launch Java from: {}", &current_java_path);

        let max_ram = config.get("max_ram").unwrap_or("2048");
        let min_ram = config.get("min_ram").unwrap_or("1024");

        let args = self.generate_arguments(&config, profile)?;

        println!("Launching with args: {:?}", &args);

        let output = Command::new(current_java_path)
            .arg(format!("-Xmx{}M", max_ram))
            .arg(format!("-Xms{}M", min_ram))
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        if !output.status.success() {
            return Err(BackendError::InstallationError(
                InstallationError::FailedToExecute(self.name.clone()),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Installations(pub Vec<Installation>);

impl Installations {
    pub fn new() -> Self {
        Installations(Vec::new())
    }

    pub fn load() -> io::Result<Self> {
        let content = fs::read_to_string(INSTALLATIONS_PATH.as_path())?;
        Ok(serde_json::from_str(&content).unwrap_or(Installations::new()))
    }

    pub fn overwrite(installations: &Installations) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(INSTALLATIONS_PATH.as_path())?;

        serde_json::to_writer_pretty(file, &installations)?;

        Ok(())
    }

    pub fn add(installation: &Installation) -> io::Result<()> {
        let mut existing_installations = Self::load()?;

        if !existing_installations
            .0
            .iter()
            .any(|existing| existing.name == installation.name)
        {
            existing_installations.0.push(installation.clone());
        }

        Installations::overwrite(&existing_installations)?;

        Ok(())
    }

    pub fn remove(name: &str) -> io::Result<()> {
        let mut existing_installations = Self::load()?;

        existing_installations
            .0
            .retain(|existing| existing.name != name);

        Installations::overwrite(&existing_installations)?;

        fs::remove_dir_all(INSTALLATIONS_DIR.join(name))?;

        Ok(())
    }

    fn find_in_installations_dir(name: &str) -> Result<Installation, BackendError> {
        let path = Path::new(&INSTALLATIONS_DIR.as_path()).join(&name);

        if path.exists() && path.is_dir() {
            let instance = Installation::get_installation_from_dir(name)?;
            Installations::add(&instance)?;

            return Ok(instance);
        }

        Err(BackendError::InstallationError(
            InstallationError::InstallationNotFound(name.to_string()),
        ))
    }

    pub fn find(name: &str) -> Result<Installation, BackendError> {
        let installations = Self::load()?;

        if let Some(installation) = installations
            .0
            .into_iter()
            .find(|installation| installation.name == name)
        {
            Ok(installation)
        } else {
            Self::find_in_installations_dir(name)
        }
    }

    pub fn load_all_installations() -> Result<Installations, BackendError> {
        let mut names = Vec::new();
        let mut installations: Installations = Installations(Vec::new());

        for entry in fs::read_dir(INSTALLATIONS_DIR.as_path())? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                if let Some(folder_name_str) = entry_path.file_name().and_then(|f| f.to_str()) {
                    names.push(folder_name_str.to_string());
                }
            }
        }

        for name in names {
            installations.0.push(Installations::find(&name)?);
        }

        Ok(installations)
    }
}
