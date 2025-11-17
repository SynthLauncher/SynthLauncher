use crate::{
    config::InstanceConfig,
    instances::{instance_metadata::InstanceMetadata, InstanceManager},
    minecraft::{install_client, minecraft_version::LoadedMinecraftVersion},
};
use sl_java_manager::MULTI_PATH_SEPARATOR;
use sl_meta::{minecraft::loaders::vanilla::Client, minecraft::version_manifest::VersionType};
use sl_player::PlayerData;
use sl_utils::{dlog, errors::BackendError, log, progress::ProgressReceiver, wlog};

use chrono::DateTime;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};
use tokio::{io::AsyncRead, process::Command};

// Represents a loaded instance of Minecraft with its configurations and things required for launching
pub struct LoadedInstance<'a> {
    manager: &'a InstanceManager<'a>,
    loaded_version: LoadedMinecraftVersion,
    instance_metadata: InstanceMetadata,
    config: InstanceConfig,
    instance_path: PathBuf,
}

impl<'a> LoadedInstance<'a> {
    pub(super) const fn new(
        manager: &'a InstanceManager<'a>,
        instance_metadata: InstanceMetadata,
        instance_dir: PathBuf,
        loaded_version: LoadedMinecraftVersion,
        config: InstanceConfig,
    ) -> Self {
        Self {
            manager,
            instance_metadata,
            config,
            instance_path: instance_dir,
            loaded_version,
        }
    }

    const fn mc_version(&self) -> &str {
        self.instance_metadata.mc_version.as_str()
    }

    const fn mc_release_time(&self) -> &str {
        self.instance_metadata.mc_release_time.as_str()
    }

    const fn mc_type(&self) -> VersionType {
        self.instance_metadata.mc_type
    }

    const fn client_json(&self) -> &Client {
        self.loaded_version.client_json()
    }

    fn minecraft_jar_path(&self) -> &Path {
        self.loaded_version.client_jar_path()
    }

    fn instance_dir(&self) -> &Path {
        &self.instance_path
    }

    /// Downloads the required files for executing minecraft for this instance, only downloads corrupted or non existing files
    ///
    /// the reason why the download operation is not a part of the init, is because this operation results is not part of the instance's memory representation
    /// so i think splitting them makes the code more maintainable
    async fn download_minecraft(&self, progress: &ProgressReceiver) -> Result<(), BackendError> {
        // will automatically perform hash verification and only re install corrupted files
        install_client(
            self.manager.requester(),
            progress,
            &self.loaded_version.client_json(),
            &self.loaded_version.client_jar_path(),
            &self.instance_path,
            self.manager.assets_root(),
            self.manager.libs_root(),
        )
        .await?;
        Ok(())
    }

    /// Generates the classpath for executing minecraft for this instance
    fn generate_classpath(&self) -> String {
        let libs_root = self.manager.libs_root();

        let client = self.client_json();
        let libs = client.libraries();

        let mut classpath = Vec::new();
        for lib in libs {
            if let Some(ref native) = lib.native_from_platform() {
                let path = native.path.as_ref().unwrap();

                let full_path = libs_root.join(path);
                classpath.push(format!("{}", full_path.display()));
            }

            if let Some(ref artifact) = lib.downloads.artifact {
                let path = artifact.path.as_ref().unwrap();

                let full_path = libs_root.join(path);
                classpath.push(format!("{}", full_path.display()));
            }
        }

        let minecraft_jar = self.minecraft_jar_path();
        classpath.push(minecraft_jar.to_string_lossy().into_owned());
        classpath.join(MULTI_PATH_SEPARATOR)
    }

    // Thanks MrMayMan
    fn generate_sound_arguments(&self, jvm_args: &mut Vec<String>) {
        if self.mc_type() == VersionType::OldBeta || self.mc_type() == VersionType::OldAlpha {
            jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());

            if self.mc_version().starts_with("c0.") {
                // Classic
                jvm_args.push("-Dhttp.proxyPort=11701".to_owned());
            } else if self.mc_type() == VersionType::OldAlpha {
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
            let release = DateTime::parse_from_rfc3339(&self.mc_release_time()).unwrap();

            if release <= v1_5_2 {
                // 1.0 - 1.5.2
                jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());
                jvm_args.push("-Dhttp.proxyPort=11707".to_owned());
            }
        }
    }

    async fn generate_arguments(
        &self,
        player_username: &str,
        player_data: &PlayerData,
    ) -> Result<Vec<String>, BackendError> {
        let classpath = self.generate_classpath();
        let game_dir = self.instance_dir();

        let natives_dir = game_dir.join(".natives");

        let client = self.client_json();

        let raw_args = &client.arguments;
        let (mut jvm_args, mut game_args) = raw_args.clone().into_raw();

        let regex = regex::Regex::new(r"\$\{(\w+)\}").expect("Failed to compile regex!");

        self.generate_sound_arguments(&mut jvm_args);
        let assets_root = self.manager.assets_root();
        let libs_root = self.manager.libs_root();

        let fmt_arg = |arg: &str| {
            Some(match arg {
                "game_directory" => game_dir.to_str()?,
                "assets_root" | "game_assets" => assets_root.to_str()?,
                "assets_index_name" => &client.assets,
                "version_name" => self.mc_version(),
                "classpath" => classpath.as_str(),
                "natives_directory" => natives_dir.to_str()?,
                "auth_uuid" => &player_data.id,
                "auth_access_token" => &player_data.access_token,
                "auth_player_name" => &player_username,
                "clientid" => "74909cec-49b6-4fee-aa60-1b2a57ef72e1", // Please don't steal :(
                "version_type" => "SL",
                "library_directory" => libs_root.to_str()?,
                "classpath_separator" => MULTI_PATH_SEPARATOR,
                "launcher_name" => "SynthLauncher",
                "launcher_version" => "1.0",
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

        // !!!DO NOT REMOVE!!!
        // jvm_args.push("-javaagent:/home/stierprogrammer/Desktop/synthlauncher/assets/scripts/authlib-injector-1.2.5.jar=http://0.0.0.0:8000/".to_string());
        // jvm_args.push("-Dauthlibinjector.noShowServerName".to_string());

        jvm_args.push(client.main_class.clone());
        Ok([jvm_args, game_args].concat())
    }

    fn last_log_path(&self) -> PathBuf {
        self.instance_path.join("logs/latest-run.log")
    }

    #[must_use = "must wait on child to exit"]
    /// Performs the execution of the instance.
    ///
    /// # Returns
    /// - Ok((child, reader)) reader is a pipe reader that can be used to read the output of the instance (stderr and stdout)
    /// - Err(BackendError) if the instance could not be executed
    pub async fn execute(
        self,
        progress: ProgressReceiver,
    ) -> Result<(tokio::process::Child, impl AsyncRead), BackendError> {
        // the reason why the download operation is done here is to ensure that the files are available before executing the instance.
        // AND THE REASON WHY YOU DON'T LEAVE CALLING THIS TO THE CALLER OF THE EXECUTE METHOD is because it is just better and cleaner,
        // you should aim to ensure that the caller will get a compile time error instead of causing a runtime bug and each exported function should be self-contained.
        self.download_minecraft(&progress).await?;

        let accounts = self.manager.try_load_accounts().await?;
        let (name, data) = accounts.get_current();

        log!(
            "Executing instance '{}' with type '{:?}', using profile '{}'",
            self.instance_metadata.name,
            self.instance_metadata.mod_loader,
            name
        );

        let current_java_path = self.config.java.java();

        log!("Using Java path: {}", current_java_path.display());

        let max_ram = self.config.java.max_ram();
        let min_ram = self.config.java.min_ram();

        dlog!("min_ram: {}, max_ram: {}", min_ram, max_ram);

        let args = self.generate_arguments(&name, &data).await?;

        dlog!("Launching with args: {:?}", &args);

        let log_path = self.last_log_path();
        tokio::fs::create_dir_all(log_path.parent().unwrap()).await?;

        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)?;

        dlog!(
            "Using Java: {}, logging to {}",
            current_java_path.display(),
            log_path.display()
        );

        let child = Command::new(current_java_path)
            .arg(format!("-Xmx{}M", max_ram))
            .arg(format!("-Xms{}M", min_ram))
            .args(args)
            .stdout(log_file.try_clone()?)
            .stderr(log_file.try_clone()?)
            .current_dir(self.instance_path)
            .spawn()?;

        let log_file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(log_path)
            .await?;
        Ok((child, log_file))
    }
}
