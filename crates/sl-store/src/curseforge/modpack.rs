use std::{fs::File, io::BufReader, path::Path, str::FromStr};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::instances::{instance_metadata::ModLoader, InstanceManager};
use sl_utils::{errors::BackendError, requester::Requester, zip::ZipExtractor};
use tempdir::TempDir;

use crate::curseforge::api::project::get_curseforge_project_file;

const MODPACK_MANIFEST_NAME: &str = "manifest.json";

#[derive(Debug, Deserialize)]
struct CurseforgeModLoader {
    id: String,
    // pub primary: bool,
}

impl CurseforgeModLoader {
    pub fn extract_mod_loader(&self) -> (ModLoader, &str) {
        let (mod_loader, version) = self.id.rsplit_once("-").expect("ModLoader id must have a hyphen!");
        (ModLoader::from_str(mod_loader).unwrap(), version)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Minecraft {
    version: String,
    mod_loaders: Vec<CurseforgeModLoader>,
}

#[derive(Debug, Deserialize, Clone)]
struct ModpackFile {
    #[serde(rename = "projectID")]
    project_id: u32,
    #[serde(rename = "fileID")]
    file_id: u32,
    // pub required: bool,
}

#[derive(Debug, Deserialize)]
struct ModpackManifest {
    minecraft: Minecraft,
    name: String,
    // pub author: String,
    files: Vec<ModpackFile>,
}

async fn read_modpack_manifest(modpack_path: &Path) -> Result<ModpackManifest, BackendError> {
    let bytes = tokio::fs::read(modpack_path.join(MODPACK_MANIFEST_NAME)).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

async fn download_modpack_file(
    requester: &Requester,
    mods_folder: &Path,
    modpack_file: &ModpackFile,
) -> Result<(), BackendError> {
    let project_file =
        get_curseforge_project_file(requester, modpack_file.project_id, modpack_file.file_id).await?;
    let mod_path = mods_folder.join(&project_file.file_name());

    requester
        .builder()
        .download_to(&project_file.download_url(), &mod_path)
        .await?;

    Ok(())
}

async fn download_modpack_files(
    requester: &Requester,
    instance_path: &Path,
    modpack_files: Vec<ModpackFile>,
) -> Result<(), BackendError> {
    let tasks = FuturesUnordered::new();

    for modpack_file in &modpack_files {
        tasks.push(download_modpack_file(&requester, &instance_path, &modpack_file));
    }

    let futures = tasks.collect::<Vec<_>>().await;
    for result in futures {
        result?;
    }

    Ok(())
}

// FIXME: The function starts unzipping before the zip is even valid
pub async fn download_curseforge_modpack<'a>(
    instance_manager: &mut InstanceManager<'a>,
    mod_id: u32,
    file_id: u32,
) -> Result<(), BackendError> {
    let project_file =
        get_curseforge_project_file(instance_manager.requester(), mod_id, file_id).await?;
    let tmp_dir = TempDir::new(&project_file.file_name())?;
    let modpack_zip_path = tmp_dir.path().join(project_file.file_name());

    instance_manager
        .requester()
        .builder()
        .download_to(&project_file.download_url(), &modpack_zip_path)
        .await?;

    let extractor = ZipExtractor::new(BufReader::new(File::open(modpack_zip_path)?));
    extractor.extract(tmp_dir.path())?;

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
