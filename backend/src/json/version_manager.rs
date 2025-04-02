use std::path::PathBuf;

use synthlauncher_meta::json::version_manifest::VersionManifest;

use crate::{json::{manifest::{self, download_version}, installations::Installations}, utils::errors::BackendError, INSTALLATIONS_DIR};

pub async fn install_or_launch_version(version: &str, java_version: u8) -> Result<(), BackendError> {
    let mut installations = Installations::load();
    
    // Check if version is already installed
    if let Some(installation) = installations.get(version) {
        // Update last used timestamp and launch
        installations.update_last_used(version)?;
        // TODO: Add launch logic here
        return Ok(());
    }

    // Version not installed, need to download and install
    let manifest = manifest::read();
    let version_data = download_version(&manifest, version).await?;

    // Create version directory
    let version_path = INSTALLATIONS_DIR.join(version);
    std::fs::create_dir_all(&version_path)?;

    // Save version data
    std::fs::write(
        version_path.join("version.json"),
        version_data
    )?;

    // TODO: Download game files and libraries
    // This will involve parsing version.json and downloading all required files

    // Record installation
    installations.add(
        version.to_string(),
        version_path,
        java_version
    )?;

    Ok(())
}

pub fn get_installation_path(version: &str) -> Option<PathBuf> {
    let installations = Installations::load();
    installations.get(version).map(|i| i.path.clone())
}