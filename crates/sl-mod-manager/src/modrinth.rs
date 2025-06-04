use reqwest::Client;
use sl_core::{instance::{Instance, InstanceType}, INSTANCES_DIR};
use sl_utils::utils::errors::BackendError;
use synrinth::{api::{mrpack::{download_modpack_files, read_modpack_file, unpack_modpack}, project::{download_project_file, query_project_version}}, models::{mrpack::DependencyID, project::Project}};

// !!! UNFINISHED DO NOT TOUCH OR COPY THIS CODE
// !!! THERE MAY BE A LOT OF PROBLEMS HERE
pub async fn create_modpack_instance(
    client: &Client,
    project: &Project,
    version: &str
) -> Result<(), BackendError> {
    let project_version = query_project_version(&client, &project.slug, version).await?;
    let name = &project.slug;
    let project_file = &project_version.files[0];
    let loader = &project_version.loaders[0];
    let instance_path = INSTANCES_DIR.join(name);
    
    std::fs::create_dir_all(&instance_path)?;

    let loader = DependencyID::from(loader.as_str());
    let vanilla = DependencyID::Minecraft;

    let path = download_project_file(&client, &project_file, &instance_path).await?;
    unpack_modpack(&path, &instance_path).await?;
    
    let mrpack = read_modpack_file(&instance_path).await?;
    let loader_version = mrpack.dependencies.get(&loader).unwrap();
    let vanilla_version = mrpack.dependencies.get(&vanilla).unwrap();

    let mut instance = Instance::new(&name, &vanilla_version, InstanceType::Fabric, None)?;
    instance.install().await?;
    instance.install_loader(loader_version).await?;
    let modpack_files = &mrpack.files;

    download_modpack_files(&client, &instance_path, modpack_files).await?;
    Ok(())
}

