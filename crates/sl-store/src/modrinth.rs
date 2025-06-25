use reqwest::Client;
use sl_core::{
    launcher::instance::{Instance, InstanceType},
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

pub async fn search_query_default(query: QueryParams) -> Result<Search, BackendError> {
    Ok(query_search::<BackendError>(&HTTP_CLIENT, query).await?)
}

// !!! UNFINISHED DO NOT TOUCH OR COPY THIS CODE
// !!! THERE MAY BE A LOT OF PROBLEMS HERE
pub async fn create_modpack_instance(
    client: &Client,
    project: &Project,
    version: &str,
) -> Result<(), BackendError> {
    let project_version =
        query_project_version::<BackendError>(&client, &project.slug, version).await?;
    let name = &project.slug;
    let project_file = &project_version.files[0];
    let loader = &project_version.loaders[0];
    let instance_path = INSTANCES_DIR.join(name);

    std::fs::create_dir_all(&instance_path)?;

    let loader = DependencyID::from(loader.as_str());
    let vanilla = DependencyID::Minecraft;

    let path =
        download_project_file::<BackendError>(&client, &project_file, &instance_path).await?;
    unpack_modpack(&path, &instance_path).await?;

    let mrpack = read_modpack_file::<BackendError>(&instance_path)?;
    let loader_version = mrpack.dependencies.get(&loader).unwrap().clone();
    let vanilla_version = mrpack.dependencies.get(&vanilla).unwrap();

    let _ = Instance::create(
        &name,
        &vanilla_version,
        InstanceType::Fabric,
        Some(loader_version),
        None,
    )?;
    let modpack_files = &mrpack.files;

    download_modpack_files::<BackendError>(&client, &instance_path, modpack_files).await?;
    Ok(())
}
