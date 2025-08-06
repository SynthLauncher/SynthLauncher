use std::path::Path;

use sl_core::{launcher::instances::{content_caching::{cache_mod_for_instance, ContentSource, ModData}, instance_metadata::InstanceMetadata}, INSTANCES_DIR, REQUESTER};
use sl_utils::errors::BackendError;
use tempdir::TempDir;

use crate::curseforge::{api::project::{get_curseforge_project_file, CurseforgeProjectVersion}, modpack::{download_modpack_files, read_modpack_manifest, unzip_modpack}};

pub mod api;
pub mod modpack;

pub async fn download_curseforge_modpack(
    mod_id: u32,
    file_id: u32
) -> Result<(), BackendError> {
    let project_file = get_curseforge_project_file(mod_id, file_id).await?;
    let tmp_dir = TempDir::new(&project_file.data.file_name)?;
    let modpack_zip_path = tmp_dir.path().join(project_file.data.file_name);

    REQUESTER
        .builder()
        .download_to(&project_file.data.download_url, &modpack_zip_path)
        .await?;

    // sleep(std::time::Duration::from_secs(2));

    unzip_modpack(&modpack_zip_path, &tmp_dir.path()).await?;

    let manifest = read_modpack_manifest(&tmp_dir.path()).await?;
    let (mod_loader, mod_loader_version) = manifest.minecraft.mod_loaders[0].extract_mod_loader();
    let mc_version = manifest.minecraft.version;

    InstanceMetadata::create(
        manifest.name.clone(), 
        &mc_version, 
        mod_loader, 
        Some(mod_loader_version.to_string()), 
        None
    ).await?;

    download_modpack_files(&*INSTANCES_DIR.join(&manifest.name), manifest.files).await?;
    
    Ok(())
}


pub async fn download_curseforge_project(
    instance_path: &Path,
    project_file: CurseforgeProjectVersion
) -> Result<(), BackendError> {
    let path = instance_path.join("mods").join(&project_file.file_name);
    
    let mod_data = ModData::new(project_file.file_name.clone(), Some(project_file.hashes[0].value.clone()), ContentSource::Curseforge);
    cache_mod_for_instance(&instance_path, project_file.file_name, mod_data).await?;

    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    REQUESTER
        .builder()
        .download_to(&project_file.download_url, &path)
        .await?;

    Ok(())
}
