use std::fs;

use synthlauncher_meta::json::version_manifest::VersionManifest;

use crate::{utils, LAUNCHER_DIR};

pub struct Manifest {
    manifest: VersionManifest
}

async fn fetch_version_manifest() -> VersionManifest {
    let path = LAUNCHER_DIR.join("version_manifest.json");

    let res = utils::download::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json").await;

    if let Ok(res) = res {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create the dir");
        }
        fs::write(&path, res).expect("Failed writing into the file: version_manifest.json");
    }

    let buffer = fs::read_to_string(path).expect("Failed reading the file: version_manifest.json");
    serde_json::from_str(buffer.as_str()).expect("Failed parsing file: version_manifest.json")
}

