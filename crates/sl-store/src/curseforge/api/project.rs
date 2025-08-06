use serde::{Deserialize, Serialize};
use sl_core::REQUESTER;
use sl_utils::errors::BackendError;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeProjectFile {
    pub file_name: String,
    pub download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryProjectFileResponse {
    pub data: CurseforgeProjectFile,
}

pub async fn get_curseforge_project_file(
    mod_id: u32,
    file_id: u32,
) -> Result<QueryProjectFileResponse, BackendError> {
    let url = format!("https://api.curseforge.com/v1/mods/{mod_id}/files/{file_id}"); 
    Ok(REQUESTER.get_json(&url).await?)
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

#[derive(Debug, Deserialize)]
pub struct QueryProjectVersions {
    pub data: Vec<CurseforgeProjectVersion>,
}

/// 1 = Forge; 4 = Fabric; 5 = Quilt; 6 = NeoForge 
pub async fn get_curseforge_project_files(
    mod_id: u32,
    game_version: &str,
    loader: u8
) -> Result<QueryProjectVersions, BackendError> {
    let url = format!("https://api.curseforge.com/v1/mods/{mod_id}/files?modLoaderType={loader}&gameVersion={game_version}");
    Ok(REQUESTER.get_json(&url).await?)
}
