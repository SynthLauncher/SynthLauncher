use std::{fmt::Display, io};

use serde::{de::Visitor, Deserialize, Deserializer};

use crate::minecraft::loaders::vanilla::{Arguments, Client, Library};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeoForgeVersion {
    mc_major_version: String,
    mc_minor_version: String,
    build_number: u16,
    beta: bool,
}

impl NeoForgeVersion {
    pub fn from_str(str: &str) -> Self {
        let mut parts = str.split('.');
        let mc_major_version = parts
            .next()
            .expect("no major minecraft version in neoforge version")
            .to_string();
        let mc_minor_version = parts
            .next()
            .expect("no minor minecraft version in neoforge version")
            .to_string();

        let neoforge_build = parts
            .next()
            .expect("no neoforge build info in neoforge version");

        let mut neoforge_build = neoforge_build.split("-");

        let build_number = neoforge_build
            .next()
            .expect("no build number in neoforge version");

        let build_number = build_number.parse::<u16>().expect("invalid build number");
        let is_beta = neoforge_build.next().is_some_and(|s| s == "beta");

        Self {
            mc_major_version,
            mc_minor_version,
            build_number: build_number,
            beta: is_beta,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}.{}.{}{}",
            self.mc_major_version,
            self.mc_minor_version,
            self.build_number,
            if self.beta { "-beta" } else { "" }
        )
    }

    /// Returns the URL for the installer jar file and the file name, for the given neoforge version
    pub fn installer_url(&self) -> (String, String) {
        let s_version = self.to_string();
        let jar_file_name = format!("neoforge-{}-installer.jar", s_version);

        (
            format!(
                "https://maven.neoforged.net/releases/net/neoforged/neoforge/{}/{}",
                s_version, jar_file_name,
            ),
            jar_file_name,
        )
    }
}

impl Display for NeoForgeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}{}",
            self.mc_major_version,
            self.mc_minor_version,
            self.build_number,
            if self.beta { "-beta" } else { "" }
        )
    }
}

impl<'de> Deserialize<'de> for NeoForgeVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NeoForgeVersionVisitor;

        impl<'de> Visitor<'de> for NeoForgeVersionVisitor {
            type Value = NeoForgeVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string in the format 'mcmajor.mcminor.neoforgebuild[-beta]'")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NeoForgeVersion::from_str(v))
            }
        }

        deserializer.deserialize_str(NeoForgeVersionVisitor)
    }
}

#[derive(Deserialize)]
pub struct NeoForgeReleases {
    versions: Vec<NeoForgeVersion>,
}

impl NeoForgeReleases {
    /// Downloads the NeoForge versions JSON file from the Forge website.using the given `do_request` function
    pub async fn download<E>(
        do_request: impl AsyncFnOnce(&str) -> Result<Vec<u8>, E>,
    ) -> Result<Self, E>
    where
        E: From<io::Error>,
    {
        const VERSIONS_JSON_URL: &str =
            "https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge";
        let bytes = do_request(VERSIONS_JSON_URL).await?;
        Ok(serde_json::from_slice(&bytes).map_err(|e| Into::<io::Error>::into(e))?)
    }

    pub fn latest_beta(
        &self,
        mc_major_version: &str,
        mc_minor_version: &str,
    ) -> Option<&NeoForgeVersion> {
        self.versions.iter().rev().find(|v| {
            v.mc_major_version == mc_major_version
                && v.mc_minor_version == mc_minor_version
                && v.beta
        })
    }

    pub fn latest_stable(
        &self,
        mc_major_version: &str,
        mc_minor_version: &str,
    ) -> Option<&NeoForgeVersion> {
        self.versions.iter().rev().find(|v| {
            v.mc_major_version == mc_major_version
                && v.mc_minor_version == mc_minor_version
                && !v.beta
        })
    }

    pub fn latest(
        &self,
        mc_major_version: &str,
        mc_minor_version: &str,
    ) -> Option<&NeoForgeVersion> {
        self.latest_stable(mc_major_version, mc_minor_version)
            .or_else(|| self.latest_beta(mc_major_version, mc_minor_version))
    }

    /// Checks if the given Minecraft and NeoForge versions combination is valid.
    pub fn validate(&self, mc_version: &str, neoforge_version: &str) -> bool {
        let mut spilt = mc_version.split('.');
        let _ = spilt.next().unwrap();

        let major_version = spilt.next().unwrap();
        let minor_version = spilt.next().unwrap();
        let neoforge_version = NeoForgeVersion::from_str(neoforge_version);
        neoforge_version.mc_major_version == major_version
            && neoforge_version.mc_minor_version == minor_version
            && self.versions.iter().any(|v| *v == neoforge_version)
    }

    pub fn latest_from_mc_version(&self, mc_version: &str) -> Option<&NeoForgeVersion> {
        let mut spilt = mc_version.split('.');
        let _ = spilt.next().unwrap();

        let major_version = spilt.next().unwrap();
        let minor_version = spilt.next().unwrap();

        let latest = self.latest(major_version, minor_version);
        latest
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NeoForgeLoaderProfile {
    arguments: Arguments,
    id: String,
    /// The .id of the client this extends
    inherits_from: String,
    main_class: String,

    libraries: Vec<Library>,
}

impl NeoForgeLoaderProfile {
    // TODO: really slow?
    pub fn join_client(self, mut client: Client) -> Client {
        assert_eq!(self.inherits_from, client.id);
        client.id = self.id;
        client.main_class = self.main_class;
        client.arguments = client.arguments.concat(self.arguments);

        let libraries = client.libraries.into_iter();
        let libraries =
            libraries.filter(|c| !self.libraries.iter().any(|l| l.name.is_same_type(&c.name)));

        let mut libraries = libraries.collect::<Vec<_>>();
        libraries.extend(self.libraries.into_iter());
        client.libraries = libraries;
        client
    }
}
