use std::path::{Path, PathBuf};

use sl_java_manager::jre_manifest::JreManifest;
use sl_meta::minecraft::version_manifest::VersionManifest;
use sl_utils::{elog, requester::Requester};

use crate::{
    accounts::AccountsManager, config::CONFIG_FILE_NAME, instances::InstanceManager, java,
    minecraft::version_manifest,
};
use std::env;
use tokio::sync::OnceCell;

/// Returns the default launcher directory location.
fn default_launcher_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return env::var("APPDATA")
            .map(|appdata| PathBuf::from(appdata).join("SynthLauncher"))
            .expect("%APPDATA% not found");
    }

    #[cfg(target_os = "macos")]
    {
        return env::var("HOME")
            .map(|home| {
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("SynthLauncher")
            })
            .expect("$HOME not found");
    }

    #[cfg(target_os = "linux")]
    {
        return env::var("HOME")
            .map(|home| PathBuf::from(home).join(".synthlauncher"))
            .expect("$HOME not found");
    }
}

pub struct LauncherEnv {
    root_launcher_dir: PathBuf,

    assets_dir: PathBuf,
    libs_dir: PathBuf,
    javas_dir: PathBuf,
    instances_dir: PathBuf,

    accounts_path: PathBuf,
    config_path: PathBuf,
    versions_path: PathBuf,

    version_manifest: OnceCell<VersionManifest>,
    jre_manifest: OnceCell<JreManifest>,
    config: config::Config,
    http_requester: Requester,
}

impl LauncherEnv {
    pub fn new(root_launcher_dir: PathBuf) -> Self {
        let config_path = root_launcher_dir.join(CONFIG_FILE_NAME);
        let config = crate::config::get_launcher_config(&config_path)
            .expect("failed to get the root config for the launcher");

        let assets_dir = root_launcher_dir.join("assets");
        let libs_dir = root_launcher_dir.join("libs");
        let javas_dir = root_launcher_dir.join("javas");
        let versions_path = root_launcher_dir.join("versions");
        let accounts_path = root_launcher_dir.join("profiles.json");
        let instances_dir = root_launcher_dir.join("instances");

        Self {
            root_launcher_dir,
            assets_dir,
            libs_dir,
            javas_dir,
            accounts_path,
            config_path,
            config,
            instances_dir,
            versions_path,
            jre_manifest: OnceCell::new(),
            version_manifest: OnceCell::new(),
            http_requester: Requester::new(),
        }
    }

    pub fn new_at_default() -> Self {
        Self::new(default_launcher_dir())
    }

    /// Returns the version manifest in the current launcher environment.
    pub async fn version_manifest(&self) -> &VersionManifest {
        self.version_manifest
            .get_or_init(|| async {
                let version_manifest_path = self.root_launcher_dir.join("version_manifest.json");
                match version_manifest::try_fetch_or_read(
                    &self.http_requester,
                    &version_manifest_path,
                )
                .await
                {
                    Err(e) => {
                        elog!("[FATAL] Failed to fetch version manifest: {e}");
                        panic!("Failed to fetch version manifest: {e:?}");
                    }
                    Ok(manifest) => manifest,
                }
            })
            .await
    }

    /// Returns the JRE manifest in the current launcher environment.
    pub async fn jre_manifest(&self) -> &JreManifest {
        self.jre_manifest
            .get_or_init(|| async {
                let jre_manifest_path = self.root_launcher_dir.join("jre_manifest.json");
                match java::try_fetch_jre_manifest_or_read(&self.http_requester, &jre_manifest_path)
                    .await
                {
                    Err(e) => {
                        elog!("[FATAL] Failed to fetch JRE manifest: {e}");
                        panic!("Failed to fetch JRE manifest: {e:?}");
                    }
                    Ok(manifest) => manifest,
                }
            })
            .await
    }

    pub fn root(&self) -> &Path {
        self.root_launcher_dir.as_path()
    }

    pub fn assets_path(&self) -> &Path {
        self.assets_dir.as_path()
    }

    pub fn libs_path(&self) -> &Path {
        self.libs_dir.as_path()
    }

    pub fn versions_path(&self) -> &Path {
        self.versions_path.as_path()
    }

    pub fn javas_path(&self) -> &Path {
        self.javas_dir.as_path()
    }

    pub fn instances_dir(&self) -> &Path {
        self.instances_dir.as_path()
    }

    pub const fn requester(&self) -> &Requester {
        &self.http_requester
    }

    pub const fn config(&self) -> &config::Config {
        &self.config
    }

    /// Returns the accounts manager in the current launcher environment.
    pub fn accounts<'s>(&'s self) -> AccountsManager<'s> {
        AccountsManager::new(&self.accounts_path)
    }

    /// Returns the instance manager in the current launcher environment.
    pub fn instances<'s>(&'s self) -> InstanceManager<'s> {
        InstanceManager::new(self)
    }

    /// Updates the configuration cache, should be called after any configuration change,
    pub fn refresh_config_cache(&mut self) {
        let config = crate::config::get_launcher_config(&self.config_path)
            .expect("failed to get the root config for the launcher");
        self.config = config;
    }
}
