use sl_utils::{errors::BackendError, requester::Requester};

use crate::curseforge::{
    api::{CurseforgeProjectFile, CurseforgeProjectVersion},
    CurseforgeModLoader, CurseforgeResponse,
};

pub async fn get_curseforge_project_file(
    requester: &Requester,
    mod_id: u32,
    file_id: u32,
) -> Result<CurseforgeProjectFile, BackendError> {
    let url = format!("https://api.curseforge.com/v1/mods/{mod_id}/files/{file_id}");
    let res: CurseforgeResponse<CurseforgeProjectFile> = requester.get_json(&url).await?;
    Ok(res.data)
}

pub async fn get_curseforge_project_files(
    requester: &Requester,
    mod_id: u32,
    mod_loader: CurseforgeModLoader,
    game_version: &str,
) -> Result<Vec<CurseforgeProjectVersion>, BackendError> {
    let mod_loader = mod_loader as u8;
    let url = format!("https://api.curseforge.com/v1/mods/{mod_id}/files?modLoaderType={mod_loader}&gameVersion={game_version}");
    let res: CurseforgeResponse<Vec<CurseforgeProjectVersion>> = requester.get_json(&url).await?;
    Ok(res.data)
}
