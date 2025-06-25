use std::{collections::HashMap, io};

use serde::Deserialize;

use crate::minecraft::loaders::vanilla::{Arguments, Client, Library};

#[derive(Debug, Deserialize)]
pub struct ForgeVersions {
    promos: HashMap<String, String>,
}

impl ForgeVersions {
    /// Downloads the Forge versions JSON file from the Forge website.using the given `do_request` function
    pub async fn download<E>(
        do_request: impl AsyncFnOnce(&str) -> Result<Vec<u8>, E>,
    ) -> Result<Self, E>
    where
        E: From<io::Error>,
    {
        const VERSIONS_JSON: &str =
            "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";

        let bytes = do_request(VERSIONS_JSON).await?;
        Ok(serde_json::from_slice(&bytes).map_err(|e| Into::<io::Error>::into(e))?)
    }

    /// Returns the Forge version for the given Minecraft version.
    #[must_use]
    pub fn get_forge_version(&self, minecraft_version: &str) -> Option<&str> {
        self.promos
            .iter()
            .find(|(version_mc, _)| *version_mc == &format!("{minecraft_version}-latest"))
            .map(|n| n.1.as_str())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForgeLoaderProfile {
    arguments: Arguments,
    id: String,
    /// The .id of the client this extends
    inherits_from: String,
    main_class: String,
    libraries: Vec<Library>,
}

impl ForgeLoaderProfile {
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
