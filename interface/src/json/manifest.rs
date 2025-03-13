use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    OldBeta,
    OldAlpha,
    Snapshot,
    Release
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub id: String,
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub versions: Vec<Version>
}
