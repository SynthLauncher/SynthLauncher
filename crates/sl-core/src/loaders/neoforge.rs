use std::{
    io::BufReader,
    path::{Path, PathBuf},
    process::Command,
};

use sl_java_manager::MULTI_PATH_SEPARATOR;
use sl_meta::minecraft::loaders::neoforge::{NeoForgeLoaderProfile, NeoForgeVersion};
use sl_utils::{
    dlog,
    errors::{BackendError, ForgeInstallerErr, HttpError, InstanceError},
    log,
    requester::Requester,
};
use tempfile::TempDir;
use tokio::io::AsyncWriteExt;

const NEOFORGE_JAVA_INSTALLER_SRC: &str =
    include_str!("../../../../assets/scripts/NeoForgeInstaller.java");

pub struct NeoForgeInstaller<'a> {
    requester: &'a Requester,
    libs_dir: &'a Path,

    minecraft_version: &'a str,
    version: NeoForgeVersion,

    java_path: &'a Path,
    javac_path: &'a Path,
    output_loader_json_path: &'a Path,

    cache_dir: TempDir,
    // ForgeInstaller.java
    java_forge_installer: PathBuf,
}

impl<'a> NeoForgeInstaller<'a> {
    pub async fn new(
        requester: &'a Requester,
        libs_dir: &'a Path,
        minecraft_version: &'a str,
        neoforge_version: &str,
        java_path: &'a Path,
        javac_path: &'a Path,
        output_loader_json_path: &'a Path,
    ) -> Result<Self, HttpError> {
        let neoforge_version = NeoForgeVersion::from_str(neoforge_version);

        let mut cache_dir =
            TempDir::new().expect("failed to create cache dir for installing neoforge");

        #[cfg(debug_assertions)]
        cache_dir.disable_cleanup(true);

        let java_forge_installer = cache_dir.path().join("ForgeInstaller.java");
        tokio::fs::write(&java_forge_installer, NEOFORGE_JAVA_INSTALLER_SRC).await?;

        Ok(Self {
            requester,
            libs_dir,
            minecraft_version,
            version: neoforge_version,
            cache_dir,
            java_path,
            javac_path,
            output_loader_json_path,
            java_forge_installer,
        })
    }

    async fn download(&self) -> Result<PathBuf, HttpError> {
        let (url, installer_name) = self.version.installer_url();

        let installer_path = self.cache_dir.path().join(installer_name);
        dlog!(
            "downloading neoforge installer from '{url}' to '{}'",
            installer_path.display()
        );

        self.requester
            .builder()
            .download_to(&url, &installer_path)
            .await?;

        Ok(installer_path)
    }

    /// Downloads and compiles the neoforge installer, returns the class path and the file path of the final compiled binary
    async fn compile_installer(&self) -> Result<(String, PathBuf), ForgeInstallerErr> {
        // This is just a library we link against `java_forge_installer` to get the actual installer
        let neoforge_installer_path = self.download().await?;
        let java_forge_installer = &self.java_forge_installer;

        // we link using javac
        let javac = self.javac_path;

        dlog!(
            "NeoForge: compiling the neoforge installer at {}, relinking with {}, using javac at: '{}'",
            neoforge_installer_path.display(),
            java_forge_installer.display(),
            javac.display(),
        );

        let output = Command::new(javac)
            .arg("-cp")
            .arg(&neoforge_installer_path)
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
            neoforge_installer_path.display(),
            self.cache_dir.path().display()
        );
        let compiled_file = java_forge_installer.with_extension("");
        Ok((classpath, compiled_file))
    }

    async fn install_to_cache(&self) -> Result<(), ForgeInstallerErr> {
        let (classpath, compiled_installer_path) = self.compile_installer().await?;

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

        // writing '{}' so forge doesn't complain about invalid json
        launcher_profiles.write(b"{}").await?;
        launcher_profiles_microsoft.write(b"{}").await?;

        let java = self.java_path;
        dlog!(
            "NeoForge: executing compiled neoforge class at: '{}', with java at: '{}'",
            compiled_installer_path.display(),
            java.display()
        );

        let output = Command::new(java)
            .arg("-cp")
            .arg(classpath)
            .arg(compiled_installer_path.file_name().unwrap())
            .current_dir(self.cache_dir.path())
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

            return Err(ForgeInstallerErr::JavaRunErr { stdout, stderr });
        }
        Ok(())
    }

    fn neoforge_version_name(&self) -> String {
        format!("neoforge-{}", self.version)
    }

    async fn install(self) -> Result<NeoForgeLoaderProfile, ForgeInstallerErr> {
        log!(
            "NeoForge: installing neoforge {} for minecraft version: '{}'",
            self.version,
            self.minecraft_version,
        );

        self.install_to_cache().await?;

        let cache_dir = self.cache_dir.path();
        let neoforge_libraries_path = cache_dir.join("libraries");
        let neoforge_versions_path = cache_dir.join("versions");

        // copy all the libraries neoforge installed to avoid re-installation,
        // neoforge also does some shenaganis to get some of them such as `net/minecraftforge/forge/1.21.1-52.1.1/forge-1.21.1-52.1.1-client.jar`, it doesn't have a download url...
        let mut neoforge_libraries = tokio::fs::read_dir(&neoforge_libraries_path).await?;

        while let Some(entry) = neoforge_libraries.next_entry().await.unwrap() {
            let src_path = entry.path();
            let dest_path = self.libs_dir.join(entry.file_name());

            sl_utils::fs::async_copy_dir_all(src_path, dest_path).await?;
        }

        // copy the neoforge json to the instance directory...
        let neoforge_version = self.neoforge_version_name();
        let neoforge_version_json_file_name = format!("{}.json", neoforge_version);

        let neoforge_version_path = neoforge_versions_path.join(neoforge_version);
        let neoforge_json_path = neoforge_version_path.join(neoforge_version_json_file_name);

        let loader_json_path = self.output_loader_json_path;

        dlog!(
            "NeoForge: copying '{}' to '{}'",
            neoforge_json_path.display(),
            loader_json_path.display()
        );
        tokio::fs::copy(&neoforge_json_path, &loader_json_path).await?;

        let loader_json = std::fs::File::open(loader_json_path)?;
        let loader_json_reader = BufReader::new(loader_json);
        let loader_json_instance = serde_json::from_reader(loader_json_reader)
            .map_err(|e| Into::<std::io::Error>::into(e))?;

        log!("NeoForge: Installed successfully!");
        Ok(loader_json_instance)
    }
}

pub async fn install_neoforge_loader(
    requester: &Requester,
    libs_dir: &Path,

    minecraft_version: &str,
    neoforge_version: &str,
    java_path: &Path,
    javac_path: &Path,
    output_loader_json_path: &Path,
) -> Result<NeoForgeLoaderProfile, BackendError> {
    NeoForgeInstaller::new(
        requester,
        libs_dir,
        minecraft_version,
        neoforge_version,
        java_path,
        javac_path,
        output_loader_json_path,
    )
    .await?
    .install()
    .await
    .map_err(|e| Into::<InstanceError>::into(e))
    .map_err(|e| e.into())
}
