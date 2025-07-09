use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    process::Stdio,
};

use chrono::DateTime;
use sl_meta::minecraft::{loaders::vanilla::Client, version_manifest::VersionType};
use sl_utils::{
    dlog, elog, log, errors::{BackendError, InstanceError}, wlog
};
use tokio::process::Command;

use crate::{
    launcher::{
        config::InstanceConfig,
        instances::metadata::{GameVersionMetadata, InstanceMetadata},
        player::{player_profile::PlayerProfile, player_profiles::PlayerProfiles},
    },
    minecraft::install_client,
    ASSETS_DIR, LIBS_DIR, MULTI_PATH_SEPARATOR,
};

// Represents a loaded instance of Minecraft with its configurations and things required for launching
pub struct LoadedInstance {
    pub instance_metadata: InstanceMetadata,
    pub config: InstanceConfig,
    pub client: Client,
    pub minecraft_jar_path: PathBuf,
    pub instance_path: PathBuf,
}

impl LoadedInstance {
    const fn game_metadata(&self) -> &GameVersionMetadata {
        &self.instance_metadata.game_metadata
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
        if self.game_metadata().r#type == VersionType::OldBeta
            || self.game_metadata().r#type == VersionType::OldAlpha
        {
            jvm_args.push("-Dhttp.proxyHost=betacraft.uk".to_owned());

            if self.game_metadata().version.starts_with("c0.") {
                // Classic
                jvm_args.push("-Dhttp.proxyPort=11701".to_owned());
            } else if self.game_metadata().r#type == VersionType::OldAlpha {
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
            let release = DateTime::parse_from_rfc3339(&self.game_metadata().release_time).unwrap();

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
                "game_directory" => game_dir.to_str()?,
                "assets_root" | "game_assets" => ASSETS_DIR.to_str()?,
                "assets_index_name" => &self.client.assets,
                "version_name" => &self.game_metadata().version,
                "classpath" => classpath.as_str(),
                "natives_directory" => natives_dir.to_str()?,
                "auth_uuid" => &profile.data.uuid,
                "auth_access_token" => &profile.access_token,
                "auth_player_name" => &profile.data.username,
                "clientid" => "74909cec-49b6-4fee-aa60-1b2a57ef72e1", // Please don't steal :(
                "version_type" => "SL",
                "library_directory" => LIBS_DIR.to_str()?,
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
            self.instance_metadata.name,
            self.instance_metadata.mod_loader,
            profile.data.username
        );

        let current_java_path = self.config.java.java();

        log!(
            "Using Java path: {}",
            current_java_path.display()
        );

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
                self.instance_metadata.name.clone(),
            )));
        }

        Ok(())
    }
}
