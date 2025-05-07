use std::{
    borrow::Cow,
    fs::{self, File, OpenOptions},
    future::Future,
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sl_meta::json::{
    fabric::{self, profile::FabricLoaderProfile},
    vanilla::Client,
    version_manifest::{MCVersion, VersionManifest, VersionType},
};
use sl_utils::utils::errors::{BackendError, DownloadError};

use crate::{
    config::config::Config,
    json::{client, manifest::download_version},
    ASSETS_DIR, INSTALLATIONS_DIR, INSTALLATIONS_PATH, LIBS_DIR, MULTI_PATH_SEPARATOR,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Installation {
    pub name: String,
    pub version: MCVersion,
}

impl Installation {
    pub fn new(name: String, version: String, manifest: &VersionManifest) -> Option<Self> {
        manifest
            .versions()
            .find(|x| x.id == version)
            .and_then(|version| {
                Some(Self {
                    name,
                    version: MCVersion {
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

        let mc_version = MCVersion {
            release_time: client.release_time,
            r#type: Some(client.r#type),
            version: client.id,
        };

        Ok(Self {
            name: name.to_string(),
            version: mc_version,
        })
    }

    fn dir_path(&self) -> PathBuf {
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

    pub async fn install_fabric(&mut self, loader_version: &str) -> Result<(), DownloadError> {
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
        >(&self.version.version, loader_version, make_request)
        .await?;
        let file = File::create(&path)?;
        serde_json::to_writer_pretty(file, &profile).unwrap();
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

    async fn reinit(&mut self, manifest: &VersionManifest) -> Result<Client, BackendError> {
        let client_raw = download_version(&manifest, &self.version.version).await?;
        let client: Client =
            serde_json::from_slice(&client_raw).expect("Failed to deserialize client.json!");

        let config = Config::create_config(client.java_version.as_ref().unwrap().major_version)
            .await
            .unwrap();
        let config = config.merge(Config::read_global().unwrap());
        self.override_config(config)?;

        fs::create_dir_all(self.dir_path())?;

        fs::write(self.client_json_path(), &client_raw)?;
        Ok(client)
    }

    pub async fn init(&mut self, manifest: &VersionManifest) -> Result<Client, BackendError> {
        match self.read_client() {
            Some(client) => Ok(client),
            None => self.reinit(manifest).await,
        }
    }

    pub async fn install(&mut self, manifest: &VersionManifest) -> Result<(), BackendError> {
        let client = self.init(manifest).await?;
        client::install_client(&ASSETS_DIR, &LIBS_DIR, client, self.dir_path().as_path()).await
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

    fn generate_sound_arguments(&self, jvm_args: &mut Vec<String>) {
        if self.version.r#type == Some(VersionType::OldBeta)
            || self.version.r#type == Some(VersionType::OldAlpha)
        {
            jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());

            if self.version.version.starts_with("c0.") {
                // Classic
                jvm_args.push("-Dhttp.proxyPort=11701".to_owned());
            } else if self.version.r#type == Some(VersionType::OldAlpha) {
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
            let release = DateTime::parse_from_rfc3339(&self.version.release_time).unwrap();

            if release <= v1_5_2 {
                // 1.0 - 1.5.2
                jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());
                jvm_args.push("-Dhttp.proxyPort=11707".to_owned());
            }
        }
    }

    fn generate_arguments(&self, config: &Config) -> Result<Vec<String>, BackendError> {
        let global_config = Config::read_global().unwrap();
        let client = self.read_client().expect("Failed to read client.json!");
        let classpath = self.classpath(&client);
        let game_dir = self.dir_path();
        let natives_dir = game_dir.join(".natives");

        let raw_args = client.arguments;
        let (mut jvm_args, mut game_args) = raw_args.into_raw();
        let args = vec!["--versionType".to_string(), "SynthLauncher".to_string()];

        let regex = regex::Regex::new(r"\$\{(\w+)\}").expect("Failed to compile regex!");

        self.generate_sound_arguments(&mut jvm_args);

        let fmt_arg = |arg: &str| {
            Some(match arg {
                "game_directory" => game_dir.to_str().unwrap(),
                "assets_root" | "game_assets" => ASSETS_DIR.to_str().unwrap(),
                "assets_index_name" => &client.assets,
                "version_name" => &self.version.version,
                "classpath" => classpath.as_str(),
                "natives_directory" => natives_dir.to_str().unwrap(),
                "auth_uuid" => "94240269-bb0f-4570-ab26-1e2a47dbc565",
                "auth_player_name" => global_config.get("auth_player_name").unwrap(),
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

        println!("Game args: {:?}", game_args);
        println!("Java args: {:?}", jvm_args);

        Ok([jvm_args, game_args, args].concat())
    }

    pub fn execute(&self) -> Result<(), BackendError> {
        let config = self.read_config().unwrap();

        let current_java_path = config.get("java").unwrap();

        println!("Trying to launch Java from: {}", &current_java_path);

        let max_ram = config.get("max_ram").unwrap_or("2048");
        let min_ram = config.get("min_ram").unwrap_or("1024");

        let args = self.generate_arguments(&config)?;

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
            return Err(BackendError::FailedToExecuteInstallation);
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

    pub fn load() -> Self {
        let content = fs::read_to_string(INSTALLATIONS_PATH.as_path())
            .expect("Failed to read installations.json!");
        serde_json::from_str(&content).unwrap_or(Installations::new())
    }

    pub fn add(installation: &Installation) {
        let mut existing_installations = Self::load();

        if !existing_installations
            .0
            .iter()
            .any(|existing| existing.name == installation.name)
        {
            existing_installations.0.push(installation.clone());
        }

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(INSTALLATIONS_PATH.as_path())
            .unwrap();

        serde_json::to_writer_pretty(file, &existing_installations).unwrap();
    }

    fn find_in_installations_dir(name: &str) -> Option<Installation> {
        let path = Path::new(&INSTALLATIONS_DIR.as_path()).join(&name);

        if path.exists() && path.is_dir() {
            let instance = Installation::get_installation_from_dir(name).unwrap();
            Installations::add(&instance);

            return Some(instance);
        }

        None
    }

    pub fn find(name: &str) -> Option<Installation> {
        let installations = Self::load();

        installations
            .0
            .into_iter()
            .find(|installation| installation.name == name)
            .or_else(|| Self::find_in_installations_dir(name))
    }
}
