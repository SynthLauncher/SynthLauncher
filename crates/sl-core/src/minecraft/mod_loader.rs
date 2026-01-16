use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use sl_meta::minecraft::loaders::{fabric::versions::get_fabric_versions, forge, neoforge, quilt::versions::get_quilt_versions};
use sl_utils::{errors::{BackendError, HttpError}, requester::Requester};
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    EnumString,
    Display,
    AsRefStr,
    Hash,
)]
#[strum(serialize_all = "lowercase")]
pub enum ModLoader {
    #[default]
    Vanilla,
    Fabric,
    Quilt,
    Forge,
    NeoForge,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModLoaderVersions(HashMap<ModLoader, Vec<String>>);

impl ModLoaderVersions
{
    async fn read(path: &Path) -> tokio::io::Result<Self>
    {
        let data = tokio::fs::read(path).await?;
        let parsed = serde_json::from_slice(&data)?;
        Ok(parsed)
    }

    // async fn get() -> Result<ModLoaderVersions, BackendError>
    // {

    // }

    // async fn cache_to(path: &Path, requester: &Requester) -> Result<ModLoaderVersions, HttpError>
    // {
    //     if let Some(parent) = path.parent()
    //     {
    //         tokio::fs::create_dir_all(parent).await?;
    //     }
    //     let mut file = tokio::fs::File::create(&path).await?;

    // }
}