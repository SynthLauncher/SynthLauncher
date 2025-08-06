use std::{path::Path, str::FromStr};

use futures_util::{stream::FuturesUnordered, StreamExt};
use sl_core::instances::{
    instance_metadata::ModLoader,
    InstanceManager,
};
use sl_utils::{errors::BackendError, requester::Requester};

use crate::modrinth::{
    api::{
        project::{query_project_version, resolve_mod, ResolutionState},
        ProjectType,
    },
    mrpack::{download_modpack_files, read_modrinth_index, unzip_modpack, DependencyID},
};

pub mod api;
pub mod mrpack;

pub async fn install_modpack(
    instance_manager: &mut InstanceManager<'_>,
    instances_dir: &Path,
    slug: &str,
    version: &str,
) -> Result<(), BackendError> {
    let requester = instance_manager.requester();
    let project_version = query_project_version(requester, slug, version).await?;
    let instance_dir = instances_dir.join(slug);
    let mrpack_path = instance_dir.join(&project_version.files[0].filename);

    tokio::fs::create_dir_all(&instance_dir).await?;

    requester
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

    let instance = instance_manager
        .create_instance(
            slug.to_string(),
            &mc_version,
            mod_loader,
            mod_loader_version,
            None,
        )
        .await?;

    download_modpack_files(requester, &instance_dir, &index.files).await?;

    let loaded_instance = instance.load_init(instance_manager).await?;
    loaded_instance.execute().await?;

    Ok(())
}

pub async fn install_mod_with_deps(
    requester: &Requester,
    slug: &str,
    version: &str,
    instance_path: &Path,
) -> Result<(), BackendError> {
    let project_type = ProjectType::Mod;

    let state = ResolutionState::new();

    resolve_mod(
        requester,
        &state,
        slug.to_string(),
        version.to_string(),
        "1.20.1",
        "fabric",
    )
    .await?;

    let futures = FuturesUnordered::new();

    let iter = state.resolved.iter();

    for entry in iter {
        let proj_id = entry.key().to_string();
        let ver_id = entry.value().to_string();

        futures.push(install_project(
            requester,
            proj_id,
            ver_id,
            instance_path,
            project_type,
        ));
    }

    let results = futures.collect::<Vec<_>>().await;
    for result in results {
        result?;
    }

    Ok(())
}

pub async fn install_project(
    requester: &Requester,
    slug: String,
    version: String,
    instance_path: &Path,
    project_type: ProjectType,
) -> Result<(), BackendError> {
    let project_version = query_project_version(requester, &slug, &version).await?;

    let path = match project_type {
        ProjectType::Mod => instance_path
            .join("mods")
            .join(&project_version.files[0].filename),
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
