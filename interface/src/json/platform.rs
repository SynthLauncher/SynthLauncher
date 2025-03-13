pub enum OsType {
    Windows,
    Linux,
    Osx
}

pub enum Arch {
    X86,
    X86_64,
    ARM64,
    ARM
}

pub struct Os {
    pub name: Option<OsType>,
    pub arch: Option<Arch>
}

impl Os {
    pub fn if_matches(&self) -> bool {
        (self.name.is_none() || self.name == Some(crate::OS)) && (self.arch.is_none() || self.arch == Some(crate::ARCH))
    }
}