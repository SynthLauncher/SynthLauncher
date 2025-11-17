use serde::{Deserialize, Serialize};

use crate::StoreCategory;

pub mod project;
pub mod search;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectFile {
    file_name: String,
    download_url: String,
}

impl CurseforgeProjectFile {
    pub const fn file_name(&self) -> &String {
        &self.file_name
    }

    pub const fn download_url(&self) -> &String {
        &self.download_url
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurseforgeHash {
    value: String,
    algo: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectVersion {
    id: u32,
    mod_id: u32,
    file_name: String,
    download_url: String,
    hashes: Vec<CurseforgeHash>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectAsset {
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectAuthor {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProject {
    slug: String,
    name: String,
    summary: String,
    id: u32,
    download_count: u64,
    logo: CurseforgeProjectAsset,
    authors: Vec<CurseforgeProjectAuthor>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgePagination {
    total_count: u32,
}

#[repr(u32)]
pub enum CurseforgeClassID
{
    Modpack = 4471,
    Mod = 6,
    Shaderpack = 6552,
    Resourcepack = 12
}

impl From<StoreCategory> for CurseforgeClassID {
    fn from(value: StoreCategory) -> Self {
        match value {
            StoreCategory::Modpacks => CurseforgeClassID::Modpack,
            StoreCategory::Mods => CurseforgeClassID::Mod,
            StoreCategory::Resourcepacks => CurseforgeClassID::Resourcepack,
            StoreCategory::Shaderpacks => CurseforgeClassID::Shaderpack
        }
    }
}
