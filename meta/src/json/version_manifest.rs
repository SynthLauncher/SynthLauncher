use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    OldAlpha,
    OldBeta,
    Release,
    Snapshot
}

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    pub version_type: VersionType,
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: String, // Only in version_manifest_v2, which will be using
    // TODO: Maybe turn this into a bool, because it can either be 0 or 1
    pub compliance_level: u8 // Only in version_manifest_v2, which will be using
}

#[derive(Debug, Deserialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>
}
