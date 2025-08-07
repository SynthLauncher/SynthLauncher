use serde::Deserialize;
use sl_utils::{errors::BackendError, requester::Requester};

use crate::curseforge::api::{CurseforgeProjectFile, CurseforgeProjectVersion};

/// Fetches and returns a specific file from a CurseForge project,
/// identified by the project (`mod_id`) and file (`file_id`) IDs.
pub async fn get_curseforge_project_file(
    requester: &Requester,
    mod_id: u32,
    file_id: u32,
) -> Result<CurseforgeProjectFile, BackendError> {
    let url = format!("https://api.curseforge.com/v1/mods/{mod_id}/files/{file_id}");
    let res: ProjectFileResponse = requester.get_json(&url).await?;
    Ok(res.data)
}

#[derive(Debug, Deserialize)]
struct ProjectFileResponse {
    pub data: CurseforgeProjectFile,
}

/// Fetches and returns all files for the given CurseForge project ID,
/// filtered by mod loader and Minecraft game version.
///
/// `loader` values:
/// - 1 = Forge
/// - 4 = Fabric
/// - 5 = Quilt
/// - 6 = NeoForge
pub async fn get_curseforge_project_files(
    requester: &Requester,
    mod_id: u32,
    loader: u8,
    game_version: &str,
) -> Result<Vec<CurseforgeProjectVersion>, BackendError> {
    let url = format!("https://api.curseforge.com/v1/mods/{mod_id}/files?modLoaderType={loader}&gameVersion={game_version}");
    let res: ProjectVersionsResponse = requester.get_json(&url).await?;
    Ok(res.data)
}

#[derive(Debug, Deserialize)]
struct ProjectVersionsResponse {
    pub data: Vec<CurseforgeProjectVersion>,
}
