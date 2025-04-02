use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::SystemTime,
};

use serde::{Deserialize, Serialize};
use synthlauncher_meta::json::{client::Client, version_manifest::VersionManifest};
use velcro::hash_map_from;

use crate::{
    json::{client, manifest::download_version},
    utils::errors::BackendError,
    ASSETS_DIR, INSTALLATIONS_DIR, LIBS_DIR,
};

use super::{
    config::{Config, ConfigMut},
    java::JavaInstallation,
    MULTI_PATH_SEPERATOR,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstallationMetadata {
    name: String,
    version: String,
    java_version: String,
    last_used: String,
    status: String,
}

impl InstallationMetadata {
    pub fn new(name: String, version: String) -> Self {
        let timestamp = chrono::Local::now().to_rfc3339();
        
        Self {
            name,
            version,
            java_version: "17".to_string(), // Default to Java 17
            last_used: timestamp,
            status: "installing".to_string(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallationsConfig {
    installations: Vec<InstallationMetadata>,
}

impl InstallationsConfig {
    pub fn load() -> Self {
        let config_path = INSTALLATIONS_DIR.join("installations.json");
        if let Ok(data) = fs::read_to_string(&config_path) {
            serde_json::from_str(&data).unwrap_or(Self { installations: vec![] })
        } else {
            Self { installations: vec![] }
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_path = INSTALLATIONS_DIR.join("installations.json");
        fs::create_dir_all(&*INSTALLATIONS_DIR)?;
        fs::write(config_path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn add_installation(&mut self, metadata: InstallationMetadata) {
        if let Some(existing) = self.installations.iter_mut()
            .find(|i| i.name == metadata.name && i.version == metadata.version) {
            *existing = metadata;
        } else {
            self.installations.push(metadata);
        }
        self.save().expect("Failed to save installations config");
    }

    pub fn is_installed(&self, name: &str, version: &str) -> bool {
        if let Some(installation) = self.installations.iter()
            .find(|i| i.name == name && i.version == version) {
            // Check if the installation is marked as installed and the client.jar exists
            let installation_path = INSTALLATIONS_DIR.join(name);
            let client_jar_exists = installation_path.join("client.jar").exists();
            return installation.status == "installed" && client_jar_exists;
        }
        false
    }
}

pub struct Installation {
    metadata: InstallationMetadata,
    path: PathBuf,
}

impl Installation {
    pub fn new(metadata: InstallationMetadata) -> Self {
        let path = INSTALLATIONS_DIR.join(metadata.name.clone());
        Self { metadata, path }
    }

    fn dir_path(&self) -> PathBuf {
        INSTALLATIONS_DIR.join(&self.metadata.name)
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
        let global_config = Config::read_global(&self.path)?;

        if let Some(config) = self.read_config() {
            Ok(config.merge(global_config))
        } else {
            Ok(global_config)
        }
    }

    pub fn config_mut(&mut self) -> ConfigMut {
        let config_path = self.config_path();
        self.read_config()
            .unwrap_or(Config::empty())
            .into_mut(&config_path)
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

        let java = JavaInstallation::newest();
        let java_path = java.path.as_path().to_string_lossy().to_string();
        let config = Config::new(hash_map_from! {
            "java": java_path,
        });

        self.override_config(config)?;

        fs::create_dir_all(self.dir_path())?;
        fs::write(self.client_json_path(), &client_raw)?;
        Ok(client)
    }

    pub async fn install(&mut self, manifest: &VersionManifest) -> Result<(), BackendError> {
        let mut config = InstallationsConfig::load();
        
        // Check if this version exists for any user
        let version_exists = config.installations.iter()
            .any(|i| i.version == self.metadata.version && i.status == "installed");
        let client_jar_exists = self.client_jar_path().exists();

        // If the version exists for any user or client.jar exists, create a new installation entry
        if version_exists || client_jar_exists {
            let mut installation_meta = self.metadata.clone();
            installation_meta.status = "installed".to_string();
            installation_meta.last_used = chrono::Local::now().to_rfc3339();
            config.add_installation(installation_meta);
            
            // Create user-specific directory if it doesn't exist
            fs::create_dir_all(self.dir_path())?;
            
            // If client files don't exist in user directory but exist for another user, copy them
            if version_exists {
                if let Some(existing_install) = config.installations.iter()
                    .find(|i| i.version == self.metadata.version && i.status == "installed") {
                    let existing_dir = INSTALLATIONS_DIR.join(&existing_install.name);
                    
                    // Copy client.jar if needed
                    if !client_jar_exists {
                        let existing_jar = existing_dir.join("client.jar");
                        if existing_jar.exists() {
                            fs::copy(existing_jar, self.client_jar_path())?;
                        }
                    }
                    
                    // Copy client.json
                    let existing_json = existing_dir.join("client.json");
                    if existing_json.exists() {
                        fs::copy(existing_json, self.client_json_path())?;
                    }
                }
            }
            return Ok(());
        }

        // Start fresh installation
        let mut installation_meta = self.metadata.clone();
        installation_meta.status = "installing".to_string();
        config.add_installation(installation_meta.clone());

        let client = self.init(manifest).await?;
        let result = client::install_client(&ASSETS_DIR, &LIBS_DIR, client, &self.dir_path()).await;

        if result.is_ok() {
            installation_meta.status = "installed".to_string();
            config.add_installation(installation_meta);
        }

        result
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
        let current_java_path = config.get("java")
            .ok_or_else(|| BackendError::ConfigError("Java path not found in config".to_string()))?;
        let max_ram = config.get("max_ram")
            .unwrap_or("2048"); // Default to 2GB if not specified
        let min_ram = config.get("min_ram")
            .unwrap_or("1024"); // Default to 1GB if not specified

        let args = self.generate_arguments(&config)?;

        dbg!("executing with args: {:?}", &args);

        let output = Command::new(current_java_path)
            .arg(format!("-Xmx{}M", max_ram))
            .arg(format!("-Xms{}M", min_ram))
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        if !output.status.success() {
            return Err(BackendError::JavaVersionNotFound);
        }

        Ok(())
    }
}