use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod fabric;
pub mod vanilla;
pub mod version_manifest;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OsName {
    Windows,
    Linux,
    Osx,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86,
    X86_64,
    #[serde(rename = "arm64")]
    ARM64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Os {
    pub name: Option<OsName>,
    pub arch: Option<Arch>,
}

impl Os {
    pub fn matches(&self) -> bool {
        (self.name.is_none() || self.name == Some(crate::OS))
            && (self.arch.is_none() || self.arch == Some(crate::ARCH))
    }
}

#[derive(Debug, Clone)]
pub struct JavaClassName {
    group_id: String,
    artifact_id: String,
    version: String,
}

/* 
    Example of what this does:
    Deserializing "ca.weblite:java-objc-bridge:1.1" -> "ca.weblite", "java-objc-bridge", "1.1"
*/
impl<'de> Deserialize<'de> for JavaClassName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() < 3 {
            return Err(serde::de::Error::custom("Invalid Java class name format!"));
        }

        Ok(JavaClassName {
            group_id: parts[0].to_string(),
            artifact_id: parts[1].to_string(),
            version: parts[2].to_string(),
        })
    }
}

impl Serialize for JavaClassName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!(
            "{}:{}:{}",
            self.group_id, self.artifact_id, self.version
        ))
    }
}

impl JavaClassName {
    pub fn is_same_type(&self, other: &JavaClassName) -> bool {
        self.group_id == other.group_id && self.artifact_id == other.artifact_id
    }

    pub fn into_directory_and_jar(&self) -> (PathBuf, String) {
        let directory = format!(
            "{}/{}/{}",
            self.group_id.replace('.', "/"),
            self.artifact_id.replace('.', "/"),
            self.version,
        );
        let jar = format!("{}-{}.jar", self.artifact_id, self.version);
        (directory.into(), jar)
    }
}
