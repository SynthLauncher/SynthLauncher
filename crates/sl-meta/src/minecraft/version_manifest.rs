use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
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
    pub sha1: String,
    pub compliance_level: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VersionManifest {
    latest: Latest,
    versions: Vec<Version>,
}

impl VersionManifest {
    #[inline]
    fn versions_by_type(&self, kind: VersionType) -> impl Iterator<Item = &Version> {
        self.versions.iter().filter(move |v| v.r#type == kind)
    }

    #[inline]
    pub const fn latest(&self) -> &Latest {
        &self.latest
    }

    #[inline]
    pub fn get_version_by_id(&self, id: &str) -> Option<&Version> {
        self.versions().find(|v| v.id == id)
    }

    #[inline]
    /// Returns all Minecraft versions
    pub fn versions(&self) -> impl Iterator<Item = &Version> {
        self.versions.iter()
    }

    /// Returns only Release versions of Minecraft
    pub fn release_versions(&self) -> impl Iterator<Item = &Version> {
        self.versions_by_type(VersionType::Release)
    }

    /// Returns only Alpha versions of Minecraft
    pub fn alpha_versions(&self) -> impl Iterator<Item = &Version> {
        self.versions_by_type(VersionType::OldAlpha)
    }

    /// Returns only Beta versions of Minecraft
    pub fn beta_versions(&self) -> impl Iterator<Item = &Version> {
        self.versions_by_type(VersionType::OldBeta)
    }

    /// Returns only Snapshot versions of Minecraft
    pub fn snapshot_versions(&self) -> impl Iterator<Item = &Version> {
        self.versions_by_type(VersionType::Snapshot)
    }
}
