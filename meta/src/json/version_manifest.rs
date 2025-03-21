use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    OldAlpha,
    OldBeta,
    Release,
    Snapshot,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: VersionType,
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: String, // Only in version_manifest_v2, which will be using
    // TODO: Maybe turn this into a bool, because it can either be 0 or 1
    pub compliance_level: u8, // Only in version_manifest_v2, which will be using
}

#[derive(Debug, Deserialize)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn version_type_deserialize() {
        let json = json!("old_alpha");
        let version_type: VersionType = serde_json::from_value(json).unwrap();
        assert_eq!(version_type, VersionType::OldAlpha);

        let json = json!("old_beta");
        let version_type: VersionType = serde_json::from_value(json).unwrap();
        assert_eq!(version_type, VersionType::OldBeta);

        let json = json!("release");
        let version_type: VersionType = serde_json::from_value(json).unwrap();
        assert_eq!(version_type, VersionType::Release);

        let json = json!("snapshot");
        let version_type: VersionType = serde_json::from_value(json).unwrap();
        assert_eq!(version_type, VersionType::Snapshot);
    }

    #[test]
    fn latest_version_deserialize() {
        let json = json!({
            "release": "1.16.5",
            "snapshot": "21w44a"
        });

        let latest: Latest = serde_json::from_value(json).unwrap();
        assert_eq!(latest.release, "1.16.5");
        assert_eq!(latest.snapshot, "21w44a");
    }

    #[test]
    fn version_deserialize() {
        let json = json!({
            "id": "1.16.5",
            "type": "release",
            "url": "https://example.com/1.16.5.json",
            "time": "2021-01-15T12:00:00Z",
            "releaseTime": "2021-01-15T12:00:00Z",
            "sha1": "abcd1234",
            "complianceLevel": 1
        });

        let version: Version = serde_json::from_value(json).unwrap();
        assert_eq!(version.id, "1.16.5");
        assert_eq!(version.version_type, VersionType::Release);
        assert_eq!(version.url, "https://example.com/1.16.5.json");
        assert_eq!(version.time, "2021-01-15T12:00:00Z");
        assert_eq!(version.release_time, "2021-01-15T12:00:00Z");
        assert_eq!(version.sha1, "abcd1234");
        assert_eq!(version.compliance_level, 1);
    }

    #[test]
    fn version_manifest_deserialize() {
        let json = json!({
            "latest": {
                "release": "1.16.5",
                "snapshot": "21w44a"
            },
            "versions": [
                {
                    "id": "1.16.5",
                    "type": "release",
                    "url": "https://example.com/1.16.5.json",
                    "time": "2021-01-15T12:00:00Z",
                    "releaseTime": "2021-01-15T12:00:00Z",
                    "sha1": "abcd1234",
                    "complianceLevel": 1
                },
                {
                    "id": "21w44a",
                    "type": "snapshot",
                    "url": "https://example.com/21w44a.json",
                    "time": "2021-01-15T12:00:00Z",
                    "releaseTime": "2021-01-15T12:00:00Z",
                    "sha1": "efgh5678",
                    "complianceLevel": 0
                }
            ]
        });

        let manifest: VersionManifest = serde_json::from_value(json).unwrap();
        assert_eq!(manifest.latest.release, "1.16.5");
        assert_eq!(manifest.latest.snapshot, "21w44a");
        assert_eq!(manifest.versions.len(), 2);

        let version1 = &manifest.versions[0];
        assert_eq!(version1.id, "1.16.5");
        assert_eq!(version1.version_type, VersionType::Release);
        assert_eq!(version1.url, "https://example.com/1.16.5.json");
        assert_eq!(version1.time, "2021-01-15T12:00:00Z");
        assert_eq!(version1.release_time, "2021-01-15T12:00:00Z");
        assert_eq!(version1.sha1, "abcd1234");
        assert_eq!(version1.compliance_level, 1);

        let version2 = &manifest.versions[1];
        assert_eq!(version2.id, "21w44a");
        assert_eq!(version2.version_type, VersionType::Snapshot);
        assert_eq!(version2.url, "https://example.com/21w44a.json");
        assert_eq!(version2.time, "2021-01-15T12:00:00Z");
        assert_eq!(version2.release_time, "2021-01-15T12:00:00Z");
        assert_eq!(version2.sha1, "efgh5678");
        assert_eq!(version2.compliance_level, 0);
    }
}
