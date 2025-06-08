use std::{collections::HashMap, io};

use serde::Deserialize;

#[derive(Deserialize)]
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
