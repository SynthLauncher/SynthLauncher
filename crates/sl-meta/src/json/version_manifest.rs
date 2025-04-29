use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    OldAlpha,
    OldBeta,
    Release,
    Snapshot,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    pub r#type: VersionType,
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: String, // Only in version_manifest_v2, which will be using
    // TODO: Maybe turn this into a bool, because it can either be 0 or 1
    pub compliance_level: u8, // Only in version_manifest_v2, which will be using
}

#[derive(Debug, Deserialize, Clone)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

impl VersionManifest {
    pub fn versions(&self) -> impl Iterator<Item = &Version> {
        self.versions.iter()
    }
}


