use serde::{Deserialize, Serialize};
use sl_utils::{errors::BackendError, requester::Requester};

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
    pub file_name: String,
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
    pub authors: Vec<CurseforgeProjectAuthor>,
}

#[derive(Debug, Deserialize)]
pub struct QueryProjectFileResponse {
    pub data: CurseforgeFile,
}

pub async fn query_project_file(
    requester: &Requester,
    mod_id: u32,
    file_id: u32,
) -> Result<QueryProjectFileResponse, BackendError> {
    let url = format!("https://api.curseforge.com/v1/mods/{mod_id}/files/{file_id}");

    let res = requester.get_json(&url).await?;
    Ok(res)
}

pub async fn download_project_file() -> Result<(), BackendError> {
    Ok(())
}
