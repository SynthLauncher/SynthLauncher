use serde::{Deserialize, Serialize};

use crate::StoreCategory;

pub mod project;
pub mod search;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    Modpack,
    Resourcepack,
    Shader,
}

impl From<StoreCategory> for ProjectType {
    fn from(value: StoreCategory) -> Self {
        match value {
            StoreCategory::Modpacks => ProjectType::Modpack,
            StoreCategory::Mods => ProjectType::Mod,
            StoreCategory::Resourcepacks => ProjectType::Resourcepack,
            StoreCategory::Shaderpacks => ProjectType::Shader
        }
    }
}

impl From<ProjectType> for &'static str {
    fn from(value: ProjectType) -> Self {
        match value {
            ProjectType::Modpack => "modpack",
            ProjectType::Mod => "mod",
            ProjectType::Resourcepack => "resourcepack",
            ProjectType::Shader => "shader",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthSearchHit {
    slug: String,
    title: String,
    description: String,
    project_id: String,
    project_type: ProjectType,
    downloads: u32,
    icon_url: Option<String>,
    author: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthSearchResult {
    hits: Vec<ModrinthSearchHit>,
    total_hits: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProjectDependency {
    project_id: Option<String>,
    version_id: Option<String>,
    dependency_type: String, // TODO: Maybe make this an enum
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProjectFileHashes {
    sha1: String,
    sha512: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProjectFile {
    // hashes: ModrinthProjectFileHashes,
    url: String,
    filename: String,
    // primary: bool,
    // size: u32,
    // file_type: Option<String>,
}

impl ModrinthProjectFile {
    pub const fn url(&self) ->  &String {
        &self.url
    }

    pub const fn filename(&self) -> &String {
        &self.filename
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProject {
    id: String,
    project_id: String,
    name: String,
    game_versions: Vec<String>,
    loaders: Vec<String>,
    version_number: String,
    downloads: u32,
    files: Vec<ModrinthProjectFile>,
    dependencies: Vec<ModrinthProjectDependency>,
}

impl ModrinthProject {
    pub const fn id(&self) -> &String {
        &self.id
    }

    pub const fn files(&self) -> &Vec<ModrinthProjectFile> {
        &self.files
    }
}