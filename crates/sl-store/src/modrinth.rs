use reqwest::Client;
use sl_core::{
    launcher::instance::{InstanceInfo, InstanceType},
    HTTP_CLIENT, INSTANCES_DIR,
};
use sl_utils::utils::errors::BackendError;
use synrinth::{
    api::{
        mrpack::{download_modpack_files, read_modpack_file, unpack_modpack},
        project::{download_project_file, query_project_version},
        search::query_search,
    },
    models::{
        mrpack::DependencyID,
        project::Project,
        search::{QueryParams, Search},
    },
};

pub async fn search_query_default<'a>(query: QueryParams<'a>) -> Result<Search, BackendError> {
    Ok(query_search(&HTTP_CLIENT, query).await?)
}

pub async fn create_modpack_instance(
    client: &Client,
    project: &Project,
    version: &str,
) -> Result<(), BackendError> {

    Ok(())
}
