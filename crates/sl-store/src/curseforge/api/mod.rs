use serde::{Deserialize, Serialize};

pub mod project;
pub mod search;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectFile {
    pub file_name: String,
    pub download_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurseforgeHash {
    pub value: String,
    pub algo: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectVersion {
    pub id: u32,
    pub mod_id: u32,
    pub file_name: String,
    pub download_url: String,
    pub hashes: Vec<CurseforgeHash>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectAsset {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectAuthor {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProject {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub download_count: u64,
    pub logo: CurseforgeProjectAsset,
    pub authors: Vec<CurseforgeProjectAuthor>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgePagination {
    pub total_count: u32,
}
