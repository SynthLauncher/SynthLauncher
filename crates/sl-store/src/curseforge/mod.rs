use sl_core::instances::{
    content_caching::{ContentCachingManager, ContentData, ContentSource, ContentType},
    InstanceManager,
};
use sl_utils::{errors::BackendError, requester::Requester};
use tempdir::TempDir;

use crate::curseforge::{
    api::{project::get_curseforge_project_file, CurseforgeProjectVersion},
    modpack::{download_modpack_files, read_modpack_manifest, unzip_modpack},
};

pub mod api;
pub mod modpack;

// FIXME: The function starts unzipping before the zip is even valid
pub async fn download_curseforge_modpack<'a>(
    instance_manager: &mut InstanceManager<'a>,
    mod_id: u32,
    file_id: u32,
) -> Result<(), BackendError> {
    let project_file =
        get_curseforge_project_file(instance_manager.requester(), mod_id, file_id).await?;
    let tmp_dir = TempDir::new(&project_file.file_name)?;
    let modpack_zip_path = tmp_dir.path().join(project_file.file_name);

    instance_manager
        .requester()
        .builder()
        .download_to(&project_file.download_url, &modpack_zip_path)
        .await?;

    // sleep(std::time::Duration::from_secs(2));

    unzip_modpack(&modpack_zip_path, &tmp_dir.path()).await?;

    let manifest = read_modpack_manifest(&tmp_dir.path()).await?;
    let (mod_loader, mod_loader_version) = manifest.minecraft.mod_loaders[0].extract_mod_loader();
    let mc_version = manifest.minecraft.version;

    instance_manager
        .create_instance(
            manifest.name.clone(),
            &mc_version,
            mod_loader,
            Some(mod_loader_version.to_string()),
        )
        .await?;

    download_modpack_files(
        instance_manager.requester(),
        &instance_manager.instance_dir(&manifest.name),
        manifest.files,
    )
    .await?;

    Ok(())
}

pub async fn download_curseforge_project<'a>(
    requester: &Requester,
    content_caching_manager: &ContentCachingManager<'a>,
    project_file: CurseforgeProjectVersion,
) -> Result<(), BackendError> {
    let path = content_caching_manager
        .instance_path()
        .join("mods")
        .join(&project_file.file_name);

    let mod_data = ContentData::new(
        project_file.file_name.clone(),
        Some(project_file.hashes[0].value.clone()),
        ContentSource::Curseforge,
    );
    content_caching_manager
        .cache_content(ContentType::Mod, project_file.file_name, mod_data)
        .await?;

    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    requester
        .builder()
        .download_to(&project_file.download_url, &path)
        .await?;

    Ok(())
}
