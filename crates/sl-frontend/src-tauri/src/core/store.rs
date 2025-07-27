use sl_core::launcher::instances::instance_metadata::ModLoader;
use sl_store::modrinth::api::{project::ModrinthProjectVersion, ProjectType};
use sl_utils::errors::BackendError;

pub async fn get_modrinth_project_versions_inner(
    slug: &str,
    game_version: &str,
    loader: ModLoader,
    project_type: ProjectType
) -> Result<Vec<ModrinthProjectVersion>, BackendError> {
    let versions = match project_type {
        ProjectType::Resourcepack | ProjectType::Shader => sl_store::modrinth::api::project::query_project_versions(slug, Some(game_version), None).await?,
        _ => sl_store::modrinth::api::project::query_project_versions(slug, Some(game_version), Some(&loader.to_string())).await?
    };

    Ok(versions)
}
