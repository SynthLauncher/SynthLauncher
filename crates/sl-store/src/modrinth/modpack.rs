use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::Deserialize;
use sl_core::instances::{
    content_manager::{Content, ContentManager, ContentSource, ContentType}, instance_metadata::ModLoader, InstanceManager,
};
use sl_utils::{
    errors::BackendError, fs::async_copy_dir_all, requester::Requester, zip::ZipExtractor,
};
use tempdir::TempDir;

use crate::modrinth::api::project::{get_modrinth_project, get_modrinth_project_from_hash, get_modrinth_project_version};

const MODRINTH_INDEX_NAME: &'static str = "modrinth.index.json";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum DependencyLoader {
    Forge(String),
    Neoforge(String),
    FabricLoader(String),
    QuiltLoader(String),
}

impl DependencyLoader {
    pub const fn get_loader_info(&self) -> (ModLoader, &String) {
        match self {
            Self::Forge(v) => (ModLoader::Forge, v),
            Self::FabricLoader(v) => (ModLoader::Fabric, v),
            Self::Neoforge(v) => (ModLoader::NeoForge, v),
            Self::QuiltLoader(v) => (ModLoader::Quilt, v),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Dependencies {
    minecraft: String,
    #[serde(flatten)]
    loader: DependencyLoader,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ModrinthIndex {
    name: String,
    version_id: String,
    files: Vec<ModrinthIndexFile>,
    dependencies: Dependencies,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ModrinthIndexFile {
    path: PathBuf,
    hashes: FileHashes,
    downloads: Vec<String>,
    file_size: u32, // Maybe this will be needed for progress tracking?
}

#[derive(Debug, Deserialize, Clone)]
pub struct FileHashes {
    pub sha1: String,
    pub sha512: String,
}

async fn read_modrinth_index(modpack_path: &Path) -> Result<ModrinthIndex, BackendError> {
    let bytes = tokio::fs::read(modpack_path.join(MODRINTH_INDEX_NAME)).await?;
    Ok(serde_json::from_slice(&bytes)?)
}

async fn copy_overrides(modpack_path: &Path, instance_path: &Path) -> Result<(), BackendError> {
    async_copy_dir_all(modpack_path.join("overrides"), instance_path).await?;
    Ok(())
}

async fn download_modpack_file(
    requester: &Requester,
    path: &Path,
    modpack_file: ModrinthIndexFile,
) -> Result<(PathBuf, Content), BackendError> {
    let path = path.join(&modpack_file.path);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    let project = get_modrinth_project_from_hash(&requester, &modpack_file.hashes.sha512).await?;
    let icon = get_modrinth_project(requester, &project.project_id).await?.icon_url;
    let content = Content::new(
        project.name,
        Some(modpack_file.hashes.sha1),
        Some(icon),
        ContentSource::Modrinth,
        ContentType::Mod(true)
    );
    
    requester
        .builder()
        .download_to(&modpack_file.downloads[0], &path)
        .await?;
    
    Ok((modpack_file.path, content))
}

async fn download_modpack_files(
    requester: &Requester,
    instance_path: &Path,
    content_manager: &mut ContentManager<'_>,
    modpack_files: Vec<ModrinthIndexFile>,
) -> Result<(), BackendError> {
    let tasks = FuturesUnordered::new();
    
    for modpack_file in modpack_files {
        tasks.push(download_modpack_file(
            requester,
            instance_path,
            modpack_file,
        ));
    }
    
    let results = tasks.collect::<Vec<_>>().await;
    
    for result in results {
        let (path, content) = result?;
        content_manager.add_content(path, content).await?;
    }
    
    Ok(())
}
pub async fn download_modrinth_modpack<'a>(
    instance_manager: &mut InstanceManager<'a>,
    slug: &str,
    version: &str,
) -> Result<(), BackendError> {
    let icon = get_modrinth_project(instance_manager.requester(), slug).await?.icon_url;
    let project = get_modrinth_project_version(instance_manager.requester(), slug, version).await?;

    let tmp_dir = TempDir::new(slug)?;
    let zip_path = tmp_dir.path().join(project.id);

    instance_manager
        .requester()
        .builder()
        .download_to(&project.files[0].url(), &zip_path)
        .await?;

    let extractor = ZipExtractor::new(BufReader::new(File::open(zip_path)?));
    extractor.extract(tmp_dir.path())?;

    let index = read_modrinth_index(&tmp_dir.path()).await?;
    let (mod_loader, mod_loader_version) = index.dependencies.loader.get_loader_info();
    let mc_version = index.dependencies.minecraft;

    instance_manager
        .create_instance(
            slug.to_string(),
            &mc_version,
            mod_loader,
            Some(mod_loader_version.to_string()),
        )
        .await?;

    let instance_path = &instance_manager.instance_dir(slug);
    let mut content_man = ContentManager::new(&instance_path);

    download_modpack_files(
        instance_manager.requester(),
        &instance_manager.instance_dir(slug),
        &mut content_man,
        index.files,
    )
    .await?;

    instance_manager.requester().builder().download_to(&icon, &instance_manager.icon_path(&instance_path)).await?;

    copy_overrides(tmp_dir.path(), &instance_manager.instance_dir(slug)).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use sl_core::environment::LauncherEnv;

    use super::*;

    #[tokio::test]
    async fn download_modrinth_modpack_test_1() -> Result<(), BackendError>
    {
        let env = LauncherEnv::new_at_default();
        let mut instance_man = env.instances();
        download_modrinth_modpack(&mut instance_man, "fabulously-optimized", "6sc4Yzsf").await?;
        Ok(())
    }
}
