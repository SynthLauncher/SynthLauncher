use std::fs;

use serde::Deserialize;
use synthlauncher_meta::json::version_manifest::{Version, VersionManifest};

use crate::{utils, LAUNCHER_DIR};

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub manifest: VersionManifest,
}

pub async fn fetch_version_manifest() -> VersionManifest {
    let path = LAUNCHER_DIR.join("version_manifest.json");

    println!("Path: {:?}", LAUNCHER_DIR.display());

    let res =
        utils::download::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
            .await;

    if let Ok(res) = res {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create the dir");
        }
        fs::write(&path, res).expect("Failed writing into the file: version_manifest.json");
    }

    let buffer = fs::read_to_string(path).expect("Failed reading the file: version_manifest.json");
    serde_json::from_str(buffer.as_str()).expect("Failed parsing file: version_manifest.json")
}

impl Manifest {
    pub async fn fetch() -> Self {
        let manifest = fetch_version_manifest().await;
        Self { manifest }
    }

    pub fn versions(&self) -> impl Iterator<Item = &Version> {
        self.manifest.versions.iter()
    }
}
