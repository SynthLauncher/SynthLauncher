use std::{
    path::{Path, PathBuf},
    process::Command,
};

use sl_meta::json::forge::ForgeVersions;
use sl_utils::utils::{
    self,
    errors::{BackendError, ForgeInstallerErr, HttpError, InstallationError},
};
use tempfile::TempDir;

use crate::{instance::Instance, HTTP_CLIENT, MULTI_PATH_SEPARATOR};

const FORGE_JAVA_INSTALLER_SRC: &str = include_str!("../../../assets/scripts/ForgeInstaller.java");

struct ForgeInstaller<'a> {
    instance: &'a Instance,
    short_version: String,
    norm_version: String,
    major_version: u32,
    cache_dir: TempDir,
    // ForgeInstaller.java
    java_forge_installer: PathBuf,
}

impl<'a> ForgeInstaller<'a> {
    async fn new(instance: &'a Instance) -> Result<Self, HttpError> {
        let mc_client = instance
            .read_client()
            .await
            .expect("failed to read client.json");
        let mc_version = mc_client.id;
        let forge_versions = ForgeVersions::download::<HttpError>(async |url: &str| {
            utils::download::download_bytes(url, &HTTP_CLIENT, 2, std::time::Duration::from_secs(5))
                .await
                .map(|bytes| bytes.to_vec())
        })
        .await?;

        let forge_version = forge_versions
            .get_forge_version(&mc_version)
            .expect("no forge version found for version");

        let short_version = format!("{mc_version}-{forge_version}");
        let mut major_mc_version = None;

        let norm_mc_version = {
            let dots_num = mc_version.split('.').enumerate().map(|(index, part)|
                // FIXME: make a minecraft version's type instead of dis
                if index == 0 {
                    major_mc_version = Some(part.parse::<u32>().unwrap())
                }
            )
                .count();
            if dots_num == 1 {
                format!("{mc_version}.0")
            } else {
                mc_version
            }
        };

        let norm_version = format!("{short_version}-{norm_mc_version}");
        let cache_dir = TempDir::new()
            .expect("failed to create a new temporary directory for installing forge");

        let java_forge_installer = cache_dir.path().join("ForgeInstaller.java");
        tokio::fs::write(&java_forge_installer, FORGE_JAVA_INSTALLER_SRC).await?;

        Ok(Self {
            instance,
            short_version,
            norm_version,
            cache_dir,
            java_forge_installer,
            major_version: major_mc_version.unwrap(),
        })
    }

    fn file_type(&self) -> &'static str {
        if self.major_version < 14 {
            "installer"
        } else {
            "universal"
        }
    }

    fn file_type_flipped(&self) -> &'static str {
        if self.major_version < 14 {
            "universal"
        } else {
            "installer"
        }
    }

    /// Downloads the forge installer's library and returns it's path
    async fn download(&self) -> Result<PathBuf, HttpError> {
        let (file_type, file_type_flipped) = (self.file_type(), self.file_type_flipped());
        let installer_path = self
            .cache_dir
            .path()
            .join(format!("forge-{}-{file_type}.jar", self.short_version));
        let file = tokio::fs::File::create_new(&installer_path).await?;

        self.try_downloading_from_urls(&[
            &format!("https://files.minecraftforge.net/maven/net/minecraftforge/forge/{ver}/forge-{ver}-{file_type}.jar", ver = self.short_version),
            &format!("https://files.minecraftforge.net/maven/net/minecraftforge/forge/{ver}/forge-{ver}-{file_type}.jar", ver = self.norm_version),
            &format!("https://files.minecraftforge.net/maven/net/minecraftforge/forge/{ver}/forge-{ver}-{file_type_flipped}.jar", ver = self.short_version),
            &format!("https://files.minecraftforge.net/maven/net/minecraftforge/forge/{ver}/forge-{ver}-{file_type_flipped}.jar", ver = self.norm_version),
        ], &installer_path).await?;

        file.sync_all().await?;
        Ok(installer_path)
    }

    async fn try_downloading_from_urls(&self, urls: &[&str], path: &Path) -> Result<(), HttpError> {
        for url in urls {
            let downloaded = utils::download::download_file(&HTTP_CLIENT, url, path, 3, std::time::Duration::from_secs(5)).await;
            match downloaded {
                Ok(_) => return Ok(()),
                Err(HttpError::Status(s)) if s == reqwest::StatusCode::NOT_FOUND => continue,
                Err(e) => return Err(e),
            }
        }
        Err(HttpError::Status(reqwest::StatusCode::NOT_FOUND))
    }

    /// Downloads and compiles the forge installer, returns the class path and the file path of the final compiled binary
    async fn compile_installer(&self) -> Result<(String, PathBuf), ForgeInstallerErr> {
        // This is just a library we link against `java_forge_installer` to get the actual installer
        let forge_installer_lib_path = self.download().await?;
        let java_forge_installer = &self.java_forge_installer;

        // we link using javac
        let javac = self.instance.get_javac();

        let output = Command::new(javac)
            .arg("-cp")
            .arg(&forge_installer_lib_path)
            .arg(java_forge_installer)
            .arg("-d")
            .arg(self.cache_dir.path())
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

            return Err(ForgeInstallerErr::CompileErr { stdout, stderr });
        }

        let classpath = format!(
            "{}{MULTI_PATH_SEPARATOR}{}",
            forge_installer_lib_path.display(),
            self.cache_dir.path().display()
        );
        let compiled_file = java_forge_installer.with_extension("");
        Ok((classpath, compiled_file))
    }

    async fn install(mut self) -> Result<(), ForgeInstallerErr> {
        let (classpath, compiled_path) = self.compile_installer().await?;
        println!("{classpath} => {}", compiled_path.display());
        // Create files to trick forge into thinking the cache dir is the launcher root
        tokio::fs::create_dir(self.cache_dir.path().join("launcher_profiles.json")).await?;
        tokio::fs::create_dir(
            self.cache_dir
                .path()
                .join("launcher_profiles_microsoft_store.json"),
        )
        .await?;

        let java = self.instance.get_java();

        let output = Command::new(java)
            .arg("-cp")
            .arg(classpath)
            .arg(compiled_path.file_name().unwrap())
            .current_dir(self.cache_dir.path())
            .output()?;

        self.cache_dir.disable_cleanup(true);
        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

            return Err(ForgeInstallerErr::JavaRunErr { stdout, stderr });
        }
        Ok(())
    }
}

pub async fn install_for_instance(instance: &Instance) -> Result<(), BackendError> {
    ForgeInstaller::new(instance)
        .await?
        .install()
        .await
        .map_err(|e| Into::<InstallationError>::into(e))
        .map_err(|e| e.into())
}
