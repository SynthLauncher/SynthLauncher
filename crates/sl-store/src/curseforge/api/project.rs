use serde::{Deserialize, Serialize};

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
pub struct CurseforgeFile {
    pub file_name: Option<String>,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
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
    pub screenshots: Vec<CurseforgeProjectAsset>,
    pub authors: Vec<CurseforgeProjectAuthor>,
    pub thumbs_up_count: u32,
    pub latest_files: Vec<CurseforgeFile>,
}
