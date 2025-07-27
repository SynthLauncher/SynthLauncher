use sl_core::{launcher::instances::instance_metadata::ModLoader, INSTANCES_DIR};
use sl_store::{
    curseforge::api::search::{query_curseforge_search, CurseforgeSearchResponse},
    facet_filters,
    modrinth::{api::{project::ModrinthProjectVersion, search::{query_search, Params, SearchResult}, ProjectType}, install_project},
};

use crate::core::store::get_modrinth_project_versions_inner;

#[tauri::command]
pub async fn search_modrinth_store(
    query: String,
    project_type: &str,
    page: u32,
) -> Result<SearchResult, String> {
    let filter = facet_filters!([ProjectType == project_type]);
    let params = Params::new(Some(query), Some(filter), None, Some(16), Some(16 * (page - 1)));
    let search_result = query_search(params).await.map_err(|e| e.to_string())?;

    Ok(search_result)
}

#[tauri::command]
pub async fn search_curseforge_store(
    query: &str,
    offset: u32,
    class_id: u32,
) -> Result<CurseforgeSearchResponse, String> {
    let search_result = query_curseforge_search(query, class_id, (offset - 1) * 16)
        .await
        .map_err(|x| x.to_string())?;
    Ok(search_result)
}


#[tauri::command]
pub async fn get_modrinth_project_versions(
    slug: &str,
    game_version: &str,
    loader: ModLoader,
    project_type: ProjectType
) -> Result<Vec<ModrinthProjectVersion>, String> {
    let versions = get_modrinth_project_versions_inner(slug, game_version, loader, project_type).await.map_err(|e| e.to_string())?;
    Ok(versions)
}

#[tauri::command]
pub async fn install_modrinth_project(slug: &str, version: &str, instance_name: &str, project_type: ProjectType) -> Result<(), String> {
    install_project(slug, version, &INSTANCES_DIR.join(instance_name), project_type).await.map_err(|e| e.to_string())?;

    Ok(())
}