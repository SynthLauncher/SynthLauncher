use sl_core::instances::{content_caching::{ContentCachingManager, ContentData, ContentSource, ContentType}, InstanceManager};
use sl_utils::{errors::BackendError, requester::Requester};
use tempdir::TempDir;

use crate::modrinth::{
    api::project::{
        query_project_version, get_modrinth_project_versions, ModrinthProjectVersion, ProjectType,
    },
    mrpack::{
        copy_overrides, download_modpack_files, read_modrinth_index, unzip_modpack, 
    },
};

pub mod api;
pub mod mrpack;

pub async fn download_modpack<'a>(instance_manager: &mut InstanceManager<'a>, slug: &str, version: &str) -> Result<(), BackendError> {
    let project_version = query_project_version(instance_manager.requester(), slug, version).await?;
    let tmp_dir = TempDir::new(slug)?;
    let modpack_zip_path = tmp_dir.path().join(project_version.id);

    instance_manager.requester()
        .builder()
        .download_to(&project_version.files[0].url, &modpack_zip_path)
        .await?;

    unzip_modpack(&modpack_zip_path, &tmp_dir.path()).await?;

    let index = read_modrinth_index(&tmp_dir.path()).await?;
    let (mod_loader, mod_loader_version) = index.dependencies.loader.get_loader_info();
    let mc_version = index.dependencies.minecraft;

    instance_manager.create_instance(
        slug.to_string(),
        &mc_version,
        mod_loader,
        Some(mod_loader_version.to_string()),
        None,
    )
    .await?;

    download_modpack_files(instance_manager.requester(), &instance_manager.instance_dir(slug), index.files).await?;

    copy_overrides(tmp_dir.path(), &instance_manager.instance_dir(slug))?;

    Ok(())
}

pub async fn download_project<'a>(
    requester: &Requester,
    content_caching_manager: &ContentCachingManager<'a>,
    slug: &str,
    version: &str,
    project_type: ProjectType,
) -> Result<(), BackendError> {
    let project_version = query_project_version(requester, slug, version).await?;
    let instance_path = content_caching_manager.instance_path();

    let path = match project_type {
        ProjectType::Mod => {
            let mod_data = ContentData::new(
                project_version.name,
                Some(project_version.files[0].hashes.sha512.clone()),
                ContentSource::Modrinth,
            );

            content_caching_manager.cache_content(
                ContentType::Mod, 
                project_version.files[0].filename.clone(), 
                mod_data
            ).await?;

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

    requester
        .builder()
        .download_to(&project_version.files[0].url, &path)
        .await?;

    Ok(())
}

pub async fn get_projects_versions(
    requester: &Requester,
    slug: &str,
    game_version: &str,
    loader: &str,
    project_type: ProjectType,
) -> Result<Vec<ModrinthProjectVersion>, BackendError> {
    match project_type {
        ProjectType::Resourcepack | ProjectType::Shader => {
            get_modrinth_project_versions(requester, slug, Some(game_version), None).await
        }
        ProjectType::Mod | ProjectType::Modpack => {
            get_modrinth_project_versions(requester, slug, Some(game_version), Some(loader)).await
        }
    }
}