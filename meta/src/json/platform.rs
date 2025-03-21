use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum OsName {
    Windows,
    Linux,
    Osx,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Arch {
    X86,
    X86_64,
    #[serde(rename = "arm64")]
    ARM64,
}

#[derive(Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use serde_json::from_value;
    use serde_json::json;

    use crate::json::platform::Arch;
    use crate::json::platform::Os;
    use crate::json::platform::OsName;

    #[test]
    fn deserialize_os_name() {
        let json = json!("windows");
        let os_name: OsName = from_value(json).unwrap();
        assert_eq!(os_name, OsName::Windows);

        let json = json!("linux");
        let os_name: OsName = from_value(json).unwrap();
        assert_eq!(os_name, OsName::Linux);

        let json = json!("osx");
        let os_name: OsName = from_value(json).unwrap();
        assert_eq!(os_name, OsName::Osx);
    }

    #[test]
    fn deserialize_arch() {
        let json = json!("x86");
        let arch: Arch = from_value(json).unwrap();
        assert_eq!(arch, Arch::X86);

        let json = json!("x86_64");
        let arch: Arch = from_value(json).unwrap();
        assert_eq!(arch, Arch::X86_64);

        let json = json!("arm64");
        let arch: Arch = from_value(json).unwrap();
        assert_eq!(arch, Arch::ARM64);
    }

    #[test]
    fn deserialize_os() {
        let json = json!({
            "name": "windows",
            "arch": "x86"
        });
        let os: Os = from_value(json).unwrap();
        assert_eq!(os.name, Some(OsName::Windows));
        assert_eq!(os.arch, Some(Arch::X86));

        let json = json!({
            "name": "linux", 
            "arch": "x86_64"
        });
        let os: Os = from_value(json).unwrap();
        assert_eq!(os.name, Some(OsName::Linux));
        assert_eq!(os.arch, Some(Arch::X86_64));

        let json = json!({
            "name": "osx", 
            "arch": "arm64"
        });
        let os: Os = from_value(json).unwrap();
        assert_eq!(os.name, Some(OsName::Osx));
        assert_eq!(os.arch, Some(Arch::ARM64));

        let json = json!({});
        let os: Os = from_value(json).unwrap();
        assert_eq!(os.name, None);
        assert_eq!(os.arch, None);
    }

    #[test]
    fn os_matches() {
        let os: Os = Os {
            #[cfg(target_os = "windows")]
            name: Some(OsName::Windows),

            #[cfg(target_os = "macos")]
            name: Some(OsName::Osx),

            #[cfg(target_os = "linux")]
            name: Some(OsName::Linux),

            #[cfg(target_arch = "x86_64")]
            arch: Some(Arch::X86_64),

            #[cfg(target_arch = "aarch64")]
            arch: Some(Arch::ARM64),

            #[cfg(target_arch = "x86")]
            arch: Some(Arch::X86),
        };

        assert!(os.matches());
    }
}
