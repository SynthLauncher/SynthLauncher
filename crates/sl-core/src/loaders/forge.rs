use sl_meta::minecraft::loaders::forge::ForgeVersions;
use sl_utils::utils::{
    self,
    errors::{BackendError, ForgeInstallerErr, HttpError, InstallationError},
};
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::TempDir;
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    launcher::instance::{Instance, InstanceType}, HTTP_CLIENT, LIBS_DIR, MULTI_PATH_SEPARATOR
};

const FORGE_JAVA_INSTALLER_SRC: &str = include_str!("../../../../assets/scripts/ForgeInstaller.java");

struct ForgeInstaller<'a> {
    instance: &'a Instance,
    short_version: String,
    norm_version: String,
    mc_version: &'a str,
    forge_version: String,
    major_version: u32,
    cache_dir: TempDir,
    // ForgeInstaller.java
    java_forge_installer: PathBuf,
}

impl<'a> ForgeInstaller<'a> {
    async fn new(instance: &'a Instance) -> Result<Self, HttpError> {
        let mc_version = &instance.game_info.version;
        let forge_versions = ForgeVersions::download::<HttpError>(async |url: &str| {
            utils::download::download_bytes(url, &HTTP_CLIENT, 2, std::time::Duration::from_secs(5))
                .await
                .map(|bytes| bytes.to_vec())
        })
        .await?;

        println!("{forge_versions:#?} => {mc_version}");
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
                &format!("{mc_version}.0")
            } else {
                mc_version
            }
        };

        let norm_version = format!("{short_version}-{norm_mc_version}");
        let mut cache_dir = TempDir::new()
            .expect("failed to create a new temporary directory for installing forge");

        #[cfg(debug_assertions)]
        cache_dir.disable_cleanup(true);

        let java_forge_installer = cache_dir.path().join("ForgeInstaller.java");
        tokio::fs::write(&java_forge_installer, FORGE_JAVA_INSTALLER_SRC).await?;

        Ok(Self {
            instance,
            short_version,
            norm_version,
            cache_dir,
            java_forge_installer,
            forge_version: forge_version.to_string(),
            mc_version,
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

    fn forge_version_name(&self) -> String {
        format!("{}-forge-{}", self.mc_version, self.forge_version)
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
            let downloaded = utils::download::download_file(
                &HTTP_CLIENT,
                url,
                path,
                3,
                std::time::Duration::from_secs(5),
            )
            .await;
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

    async fn install_to_cache(&self) -> Result<(), ForgeInstallerErr> {
        let (classpath, compiled_path) = self.compile_installer().await?;
        println!("{classpath} => {}", compiled_path.display());
        // Create files to trick forge into thinking the cache dir is the launcher root
        let mut launcher_profiles =
            tokio::fs::File::create_new(self.cache_dir.path().join("launcher_profiles.json"))
                .await?;
        let mut launcher_profiles_microsoft = tokio::fs::File::create_new(
            self.cache_dir
                .path()
                .join("launcher_profiles_microsoft_store.json"),
        )
        .await?;
        launcher_profiles.write(b"{}").await?;
        launcher_profiles_microsoft.write(b"{}").await?;

        let java = self.instance.get_java();

        let output = Command::new(java)
            .arg("-cp")
            .arg(classpath)
            .arg(compiled_path.file_name().unwrap())
            .current_dir(self.cache_dir.path())
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

            return Err(ForgeInstallerErr::JavaRunErr { stdout, stderr });
        }
        println!("DONE");
        Ok(())
    }

    async fn install(self) -> Result<(), ForgeInstallerErr> {
        /// Some helper function to recursively copy a directory
        fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
            std::fs::create_dir_all(&dst)?;
            for entry in std::fs::read_dir(src)? {
                let entry = entry?;
                let ty = entry.file_type()?;

                let src_path = entry.path();
                let dest_path = dst.as_ref().join(entry.file_name());

                if ty.is_dir() {
                    copy_dir_all(src_path, dest_path)?
                } else {
                    std::fs::copy(src_path, dest_path)?;
                }
            }
            Ok(())
        }

        self.install_to_cache().await?;

        let cache_dir = self.cache_dir.path();
        let forge_libraries_path = cache_dir.join("libraries");
        let forge_versions_path = cache_dir.join("versions");

        // copy all the libraries forge installed to avoid re-installation,
        // forge also does some shenaganis to get some of them such as `net/minecraftforge/forge/1.21.1-52.1.1/forge-1.21.1-52.1.1-client.jar`, it doesn't have a download url...
        let mut forge_libraries = fs::read_dir(&forge_libraries_path).await?;

        while let Some(entry) = forge_libraries.next_entry().await.unwrap() {
            let src_path = entry.path();
            let dest_path = LIBS_DIR.join(entry.file_name());

            copy_dir_all(src_path, dest_path)?;
        }

        // copy the forge json to the instance directory...
        let forge_version = self.forge_version_name();
        let forge_version_json_file_name = format!("{}.json", forge_version);

        let forge_version_path = forge_versions_path.join(forge_version);
        let forge_json_path = forge_version_path.join(forge_version_json_file_name);

        fs::copy(&forge_json_path, self.instance.loader_json_path().unwrap()).await?;

        Ok(())
    }
}

pub async fn install_forge_loader(instance: &Instance) -> Result<(), BackendError> {
    // it isn't the job of the installer to forge a working instance...
    assert_eq!(instance.instance_type, InstanceType::Forge);
    ForgeInstaller::new(instance)
        .await?
        .install()
        .await
        .map_err(|e| Into::<InstallationError>::into(e))
        .map_err(|e| e.into())
}
