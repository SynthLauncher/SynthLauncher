use std::path::Path;

use sl_core::{
    launcher::instances::{
        content_caching::{cache_mod_for_instance, ContentSource, ModData},
        instance_metadata::InstanceMetadata,
    },
    INSTANCES_DIR, REQUESTER,
};
use sl_utils::errors::BackendError;
use tempdir::TempDir;

use crate::modrinth::{
    api::project::{
        query_project_version, query_project_versions, ModrinthProjectVersion, ProjectType,
    },
    mrpack::{
        copy_overrides, download_modpack_files, read_modrinth_index, unzip_modpack, DependencyID,
    },
};

pub mod api;
pub mod dependencies;
pub mod mrpack;

pub async fn download_modpack(slug: &str, version: &str) -> Result<(), BackendError> {
    let project_version = query_project_version(slug, version).await?;
    let tmp_dir = TempDir::new(slug)?;
    let modpack_zip_path = tmp_dir.path().join(project_version.id);

    REQUESTER
        .builder()
        .download_to(&project_version.files[0].url, &modpack_zip_path)
        .await?;

    unzip_modpack(&modpack_zip_path, &tmp_dir.path()).await?;

    let index = read_modrinth_index(&tmp_dir.path()).await?;
    let (mod_loader, mod_loader_version) = index
        .get_mod_loader()
        .expect("Modpack must have mod loader!");
    let mc_version = index
        .dependencies
        .get(&DependencyID::Minecraft)
        .expect("Modpack must have a Minecraft version!");

    InstanceMetadata::create(
        slug.to_string(),
        &mc_version,
        mod_loader,
        Some(mod_loader_version.to_string()),
        None,
    )
    .await?;

    download_modpack_files(&*INSTANCES_DIR.join(slug), index.files).await?;

    copy_overrides(tmp_dir.path(), &*INSTANCES_DIR.join(slug))?;

    Ok(())
}

pub async fn download_project(
    slug: &str,
    version: &str,
    instance_path: &Path,
    project_type: ProjectType,
) -> Result<(), BackendError> {
    let project_version = query_project_version(slug, version).await?;

    let path = match project_type {
        ProjectType::Mod => {
            let mod_data = ModData::new(
                project_version.name,
                Some(project_version.files[0].hashes.sha512.clone()),
                ContentSource::Modrinth,
            );

            cache_mod_for_instance(
                &instance_path,
                project_version.files[0].filename.clone(),
                mod_data,
            )
            .await?;

            instance_path
                .join("mods")
                .join(&project_version.files[0].filename)
        }
        ProjectType::Shader => instance_path
            .join("shaderpacks")
            .join(&project_version.files[0].filename),
        ProjectType::Resourcepack => instance_path
            .join("resourcepacks")
            .join(&project_version.files[0].filename),
        ProjectType::Modpack => panic!("Modpack doesn't have a path!"),
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

pub async fn get_projects_versions(
    slug: &str,
    game_version: &str,
    loader: &str,
    project_type: ProjectType,
) -> Result<Vec<ModrinthProjectVersion>, BackendError> {
    match project_type {
        ProjectType::Resourcepack | ProjectType::Shader => {
            query_project_versions(slug, Some(game_version), None).await
        }
        ProjectType::Mod | ProjectType::Modpack => {
            query_project_versions(slug, Some(game_version), Some(loader)).await
        }
    }
}
