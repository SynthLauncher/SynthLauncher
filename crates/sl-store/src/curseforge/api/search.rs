use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub game_id: u32,
    pub search_filter: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectAsset {
    pub thumbnail_url: String,
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct ProjectAuthor {
    pub name: String
}

#[derive(Debug, Deserialize)]
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
    pub thumbs_up_count: u32
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {

}

