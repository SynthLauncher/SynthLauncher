use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum OsName {
    Windows,
    Linux,
    Osx
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86,
    X86_64,
    #[serde(rename = "arm64")]
    ARM64
}

#[derive(Debug, Deserialize)]
pub struct Os {
    pub name: Option<OsName>,
    pub arch: Option<Arch>
}

impl Os {   
   pub fn matches(&self) -> bool {
        (self.name.is_none() || self.name == Some(crate::OS)) && (self.arch.is_none() || self.arch == Some(crate::ARCH))
   } 
}