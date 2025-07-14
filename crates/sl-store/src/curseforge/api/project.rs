use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAsset {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectAuthor {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub file_name: Option<String>,
    pub download_url: Option<String>,
    pub game_versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub download_count: u64,
    pub logo: ProjectAsset,
    pub screenshots: Vec<ProjectAsset>,
    pub authors: Vec<ProjectAuthor>,
    pub thumbs_up_count: u32,
    pub latest_files: Vec<File>,
}
