use std::{collections::HashMap, fs::OpenOptions, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};
use sl_utils::dlog;

const MOD_LIST_FILE_NAME: &str = "mod_list.json";
const RESOURCEPACK_LIST_FILE_NAME: &str = "resourcepack_list.json";
const SHADERPACK_LIST_FILE_NAME: &str = "shaderpack_list.json";

#[derive(Debug, Deserialize, Serialize)]
pub enum ContentSource {
    Modrinth,
    Curseforge,
    External,
}

pub enum ContentType {
    Mod,
    Resourcepack,
    Shaderpack
}

pub struct ContentCachingManager<'a> {
    instance_path: &'a Path,
}

impl<'a> ContentCachingManager<'a> {
    pub fn new(instance_path: &'a Path) -> Self {
        Self { instance_path }
    }

    pub const fn instance_path(&self) -> &Path {
        self.instance_path
    }

    fn content_list_path(&self, content_type: &ContentType) -> PathBuf {
        match content_type {
            ContentType::Mod => self.instance_path.join(MOD_LIST_FILE_NAME),
            ContentType::Resourcepack => self.instance_path.join(RESOURCEPACK_LIST_FILE_NAME),
            ContentType::Shaderpack => self.instance_path.join(SHADERPACK_LIST_FILE_NAME)
        }
    }

    pub async fn get_content_list(&self, content_type: &ContentType) -> tokio::io::Result<ContentList> {
        let path = self.content_list_path(content_type);
        let data = tokio::fs::read(path).await?;
        Ok(serde_json::from_slice(&data).unwrap_or_else(|_| ContentList::new()))
    }

    fn save_content_list(&self, content_type: &ContentType, new_content_list: ContentList) -> std::io::Result<()> {
        let path = self.content_list_path(content_type);
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        serde_json::to_writer_pretty(file, &new_content_list)?;
        
        Ok(())
    }

    pub async fn cache_content(&self,content_type: ContentType, file_name: String, content_data: ContentData) -> tokio::io::Result<()> {
        let mut content_list = self.get_content_list(&content_type).await?;
        content_list.list.insert(file_name, content_data);
        self.save_content_list(&content_type, content_list)?;
        
        Ok(())
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ContentData {
    pub name: String,
    pub hash: Option<String>,
    pub source: ContentSource
}

impl ContentData {
    pub fn new(name: String, hash: Option<String>, source: ContentSource) -> Self {
        Self { name, hash, source }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContentList {
    scheme_version: u32,
    pub list: HashMap<String, ContentData>
}

impl ContentList {
     pub fn new() -> Self {
        Self {
            scheme_version: 0,
            list: HashMap::new(),
        }
    }
}
