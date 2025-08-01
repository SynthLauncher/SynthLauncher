use std::{path::Path, str::FromStr};

use sl_core::{
    launcher::instances::instance_metadata::{InstanceMetadata, ModLoader},
    INSTANCES_DIR, REQUESTER,
};
use sl_utils::errors::BackendError;

use crate::modrinth::{
    api::{project::query_project_version, ProjectType},
    mrpack::{download_modpack_files, read_modrinth_index, unzip_modpack, DependencyID},
};

pub mod api;
pub mod mrpack;

pub async fn install_modpack(slug: &str, version: &str) -> Result<(), BackendError> {
    let project_version = query_project_version(slug, version).await?;
    let instance_dir = INSTANCES_DIR.join(slug);
    let mrpack_path = instance_dir.join(&project_version.files[0].filename);

    tokio::fs::create_dir_all(&instance_dir).await?;

    REQUESTER
        .builder()
        .download_to(&project_version.files[0].url, &mrpack_path)
        .await?;

    unzip_modpack(&mrpack_path, &instance_dir).await?;
    let index = read_modrinth_index(&instance_dir).await?;
    let mod_loader = ModLoader::from_str(project_version.loaders[0].as_str())?;
    let mod_loader_id = DependencyID::from(project_version.loaders[0].as_str());
    let mod_loader_version = index.dependencies.get(&mod_loader_id).cloned();
    let mc_version = index
        .dependencies
        .get(&DependencyID::Minecraft)
        .unwrap_or(&project_version.game_versions[0]);

    let instance = InstanceMetadata::create(
        slug.to_string(),
        &mc_version,
        mod_loader,
        mod_loader_version,
        None,
    )
    .await?;

    download_modpack_files(&instance_dir, &index.files).await?;

    let loaded_instance = instance.load_init().await?;
    loaded_instance.execute().await?;

    Ok(())
}

pub async fn install_project(
    slug: &str,
    version: &str,
    instance_path: &Path,
    project_type: ProjectType
) -> Result<(), BackendError> {
    let project_version = query_project_version(slug, version).await?;
    
    let path = match project_type {
        ProjectType::Mod => {
            instance_path
                .join("mods")
                .join(&project_version.files[0].filename)
        },
        ProjectType::Shader => {
                instance_path
                .join("shaderpacks")
                .join(&project_version.files[0].filename)
        }
        ProjectType::Resourcepack => {
                instance_path
                .join("resourcepacks")
                .join(&project_version.files[0].filename)
        
        }
        ProjectType::Modpack => panic!("Modpack doesn't have a path!")
    };
    
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    REQUESTER
        .builder()
        .download_to(&project_version.files[0].url, &path)
        .await?;

    Ok(())
}
