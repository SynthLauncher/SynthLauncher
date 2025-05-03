#[derive(Clone)]
pub enum MinecraftVersionType {
    PreClassic,
    Classic,
    Indev,
    Infdev,
    Alpha,
    Beta,
    Release,
    AprilFools,
    // I'll implement this later
    // Misc
}

impl MinecraftVersionType {
    pub const ALL: [Self; 8] = [
        Self::PreClassic,
        Self::Classic,
        Self::Indev,
        Self::Infdev,
        Self::Alpha,
        Self::Beta,
        Self::AprilFools,
        Self::Release
    ];

    pub const SUPPORTS_SERVER: [Self; 5] = [Self::Classic, Self::Alpha, Self::Beta, Self::Release, Self::AprilFools];

    pub fn client_versions() -> Vec<Self> {
        Self::ALL.to_vec()
    }

    pub fn server_versions() -> Vec<Self> {
        Self::ALL.to_vec()
    }

    #[inline]
    fn as_str(&self) -> &'static str {
        match self {
            Self::PreClassic => "pre-classic",
            Self::Classic => "classic",
            Self::Indev => "indev",
            Self::Infdev => "infdev",
            Self::Alpha => "alpha",
            Self::Beta => "beta",
            Self::AprilFools => "april-fools",
            Self::Release => "release"
        }
    }

    pub fn get_url(&self, is_server: bool) -> String {
        format!(
            "https://vault.omniarchive.uk/archive/java/{}-{}/index.html",
            if is_server { "server" } else { "client" },
            self.as_str()
        )
    }    
}
