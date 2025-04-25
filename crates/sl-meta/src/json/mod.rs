use serde::Deserialize;

pub mod version_manifest;
pub mod fabric;
pub mod forge;
pub mod optifine;
pub mod vanilla;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OsName {
    Windows,
    Linux,
    Osx,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86,
    X86_64,
    #[serde(rename = "arm64")]
    ARM64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
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
