use std::path::Path;

use reqwest::Client;
use sl_utils::utils::errors::BackendError;
use synrinth::{api::query_search, structs::{FacetFilter, FacetOp, FacetType, ProjectFile, QueryParams, Search}};

pub async fn query_mods(client: &Client) -> Result<Search, BackendError> {
    let filter = FacetFilter {
        facet: FacetType::ProjectType,
        op: FacetOp::Eq,
        value: "mods".to_string()
    };

    let params = QueryParams {
        facets: Some(vec![vec![filter]]),
        query: None
    };

    Ok(query_search(&client, params).await?)
}

pub async fn query_resourcepacks() {

}

pub async fn query_shaderpacks() {

}

pub async fn query_modpacks() {

}

pub async fn install_mod(
    client: &Client,
    project_file: ProjectFile,
    instance_path: &Path,
) -> Result<(), BackendError> {
    let mods_dir = instance_path.join("mods");
    synrinth::api::download_project_file(&client, &project_file, &mods_dir).await?;

    Ok(())
}

pub async fn install_resourcepack(
    client: &Client,
    project_file: ProjectFile,
    instance_path: &Path,
) -> Result<(), BackendError> {
    let resourcepack_dir = instance_path.join("resourcepacks");
    synrinth::api::download_project_file(&client, &project_file, &resourcepack_dir).await?;

    Ok(())
}

pub async fn install_shaderpack(
    client: &Client,
    project_file: ProjectFile,
    instance_path: &Path,
) -> Result<(), BackendError> {
    let shaderpack_dir = instance_path.join("shaderpacks");
    synrinth::api::download_project_file(&client, &project_file, &shaderpack_dir).await?;

    Ok(())
}

pub async fn install_modpack(
    client: &Client,
    project_file: ProjectFile,
    instance_path: &Path
) {
    
}