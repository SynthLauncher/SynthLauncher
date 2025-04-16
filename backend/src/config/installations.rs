use std::{
    borrow::Cow,
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use serde::{Deserialize, Serialize};
use synthlauncher_meta::json::{client::Client, version_manifest::VersionManifest};

use crate::{
    json::{client, manifest::download_version},
    utils::errors::BackendError,
    ASSETS_DIR, INSTALLATIONS_DIR, INSTALLATIONS_PATH, LIBS_DIR,
};

use super::{config::Config, MULTI_PATH_SEPERATOR};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstallationMetadata {
    name: String,
    version: String,
}

impl InstallationMetadata {
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Installation {
    metadata: InstallationMetadata,
    path: PathBuf,
}

impl Installation {
    pub fn new(metadata: InstallationMetadata) -> Self {
        let path = INSTALLATIONS_DIR.join(metadata.name.clone());
        Self { metadata, path }
    }

    fn dir_path(&self) -> &Path {
        &self.path
    }

    fn config_path(&self) -> PathBuf {
        self.path.join("config.json")
    }

    fn client_jar_path(&self) -> PathBuf {
        self.path.join("client.jar")
    }

    fn client_json_path(&self) -> PathBuf {
        self.path.join("client.json")
    }

    fn read_config(&self) -> Option<Config> {
        let config_path = self.config_path();
        let config = fs::read_to_string(&config_path).ok()?;

        Some(serde_json::from_str(&config).expect("Failed to deserialize config.json!"))
    }

    fn override_config(&mut self, config: Config) -> Result<(), std::io::Error> {
        let installation_dir = self.dir_path();
        let config_path = self.config_path();

        fs::create_dir_all(&installation_dir)?;
        fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
        Ok(())
    }

    pub fn get_config(&self) -> Result<Config, std::io::Error> {
        let global_config = Config::read_global()?;

        if let Some(config) = self.read_config() {
            Ok(config.merge(global_config))
        } else {
            Ok(global_config)
        }
    }

    pub fn read_client(&self) -> Option<Client> {
        let data = fs::read_to_string(self.client_json_path()).ok()?;
        serde_json::from_str(&data).expect("Failed to deserialize client.json!")
    }

    pub async fn init(&mut self, manifest: &VersionManifest) -> Result<Client, BackendError> {
        match self.read_client() {
            Some(client) => Ok(client),
            None => self.reinit(manifest).await,
        }
    }

    async fn reinit(&mut self, manifest: &VersionManifest) -> Result<Client, BackendError> {
        let client_raw = download_version(&manifest, self.metadata.version()).await?;
        let client: Client =
            serde_json::from_slice(&client_raw).expect("Failed to deserialize client.json");

        let config = Config::create_config(client.java_version.as_ref().unwrap().major_version)
            .await
            .unwrap();
        let config = config.merge(Config::read_global().unwrap());
        self.override_config(config)?;

        fs::create_dir_all(self.dir_path())?;
        Installations::add(self);
        fs::write(self.client_json_path(), &client_raw)?;
        Ok(client)
    }

    pub async fn install(&mut self, manifest: &VersionManifest) -> Result<(), BackendError> {
        let client = self.init(manifest).await?;

        client::install_client(&ASSETS_DIR, &LIBS_DIR, client, self.dir_path()).await
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
        classpath.join(MULTI_PATH_SEPERATOR)
    }

    fn generate_arguments(&self, config: &Config) -> Result<Vec<String>, BackendError> {
        let global_config = Config::read_global().unwrap();
        let client = self.read_client().expect("Failed to read client.json!");
        let classpath = self.classpath(&client);
        let game_dir = self.dir_path();
        let natives_dir = game_dir.join(".natives");

        let raw_args = client.arguments;
        let (mut jvm_args, mut game_args) = raw_args.into_raw();
        let regex = regex::Regex::new(r"\$\{(\w+)\}").expect("Failed to compile regex!");

        let fmt_arg = |arg: &str| {
            Some(match arg {
                "game_directory" => game_dir.to_str().unwrap(),
                "assets_root" | "game_assets" => ASSETS_DIR.to_str().unwrap(),
                "assets_index_name" => &client.assets,
                "version_name" => &self.metadata.version(),
                "classpath" => classpath.as_str(),
                "natives_directory" => natives_dir.to_str().unwrap(),
                "auth_uuid" => "e371151a-b6b4-496a-b446-0abcd3e75ec4",
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
        Ok([jvm_args, game_args].concat())
    }

    pub fn execute(&self) -> Result<(), BackendError> {
        let config = self.get_config()?;

        let current_java_path = config.get("java").unwrap();
        println!("Trying to launch Java from: {}", current_java_path);
        let max_ram = config.get("max_ram").unwrap_or("2048");
        let min_ram = config.get("min_ram").unwrap_or("1024");

        let args = self.generate_arguments(&config)?;

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
            .any(|existing| existing.path == installation.path)
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
}
