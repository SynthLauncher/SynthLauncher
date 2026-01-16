use serde::{Deserialize, Serialize};

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
    pub const fn url(&self) -> &String {
        &self.url
    }

    pub const fn filename(&self) -> &String {
        &self.filename
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthContent {
    pub id: String,
    pub slug: String,
    pub icon_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModrinthProject {
    pub id: String,
    pub project_id: String,
    pub icon_url: Option<String>,
    pub name: String,
    game_versions: Vec<String>,
    loaders: Vec<String>,
    version_number: String,
    pub downloads: u32,
    pub files: Vec<ModrinthProjectFile>,
    dependencies: Vec<ModrinthProjectDependency>,
}
