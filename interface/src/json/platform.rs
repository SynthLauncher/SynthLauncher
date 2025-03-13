use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OsType {
    Windows,
    Linux,
    Osx,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86,
    X86_64,
    ARM64,
    ARM,
}

#[derive(Debug, Deserialize)]
pub struct Os {
    pub name: Option<OsType>,
    pub arch: Option<Arch>,
}

impl Os {
    pub fn if_matches(&self) -> bool {
        (self.name.is_none() || self.name == Some(crate::OS))
            && (self.arch.is_none() || self.arch == Some(crate::ARCH))
    }
}
