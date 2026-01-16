use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
/// Represents an enum from where the content is installed from
pub enum ContentSource {
    Modrinth,
    Curseforge,
    External,
}

#[derive(Debug, Deserialize, Serialize)]
/// Represents a type of a content
/// In Mod(bool) boolean type represents
/// if the mod is enabled or disabled
/// e.g. sodium.jar.disabled would be Mod(false)
/// however, sodium.jar would be Mod(true)
/// Minecraft automatically ignoress .jar.disabled mods
pub enum ContentType {
    Mod(bool),
    Resourcepack,
    Shaderpack,
}

#[derive(Debug, Deserialize, Serialize)]
/// Represents a content with its source, type, name and hash
/// The hash is used for exactly identifying the content
pub struct Content {
    name: String,
    hash: Option<String>,
    icon_url: Option<String>,
    source: ContentSource,
    r#type: ContentType,
}

impl Content {
    pub fn new(name: String, hash: Option<String>, icon_url: Option<String>, source: ContentSource, r#type: ContentType) -> Self {
        Self { name, hash, icon_url, source, r#type } 
    }
}

/// File name within an instance that contains a list of contents
const CONTENT_LIST_PATH: &'static str = "content_list.json";

#[derive(Debug, Deserialize, Serialize)]
/// Represents a list for contents, where a key is a file path
/// e.g. Mod Sodium: key = mods/sodium.jar
/// Note: We store the path as {folder}/{jar_file}, because
/// it is more convinient then storing the entire path;
/// at the end of the day we will still be joining it
/// with instance path
pub struct ContentList {
    scheme_version: u32,
    list: HashMap<PathBuf, Content>,
}

impl ContentList {
    pub fn new() -> Self {
        Self {
            scheme_version: 0,
            list: HashMap::new(),
        }
    }
}

/// Represents a content manager for a specific instance
pub struct ContentManager<'a> {
    instance_path: &'a Path,
    content_list: ContentList,
}

impl<'a> ContentManager<'a> {
    pub fn new(instance_path: &'a Path) -> Self {
        Self {
            instance_path,
            content_list: ContentList::new(),
        }
    }

    pub const fn instance_path(&self) -> &Path {
        self.instance_path
    }

    pub fn content_list_path(&self) -> PathBuf {
        self.instance_path().join(CONTENT_LIST_PATH)
    }

    pub async fn load_content_list(&mut self) -> tokio::io::Result<ContentList> {
        let data = tokio::fs::read(self.content_list_path())
            .await
            .unwrap_or_default();
        println!("{:?}", self.content_list_path());
        Ok(serde_json::from_slice(&data).unwrap_or(ContentList::new()))
    }

    // TODO: Perhaps make it asynchronous
    fn save_content_list(&self) -> tokio::io::Result<()> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.content_list_path())?;
        serde_json::to_writer_pretty(file, &self.content_list)?;
        Ok(())
    }

    async fn rename_mod(&self, file_path: &Path) -> tokio::io::Result<()> {
        const DISABLED_EXT: &'static str = ".disabled";
        if file_path.extension().and_then(|e| e.to_str()) == Some(DISABLED_EXT) {
            let path = file_path.with_extension("");
            tokio::fs::rename(&file_path, path).await?;
        } 
        else {
            let path = file_path.with_extension(DISABLED_EXT);
            tokio::fs::rename(file_path, path).await?;
        }

        Ok(())
    }

    pub async fn toggle_mod(&mut self, file_path: &Path) -> tokio::io::Result<()> {
        if let Some(content) = self.content_list.list.get_mut(file_path) {
            if let ContentType::Mod(content_type) = &mut content.r#type {
                *content_type = !*content_type;
                self.rename_mod(file_path).await?;
            }
        }
        Ok(())
    }

    pub async fn add_content(
        &mut self,
        file_path: PathBuf,
        content: Content,
    ) -> tokio::io::Result<()> {
        self.content_list.list.insert(file_path, content);
        self.save_content_list()?;
        Ok(())
    }

    pub async fn remove_content(&mut self, file_path: &Path) -> tokio::io::Result<()> {
        self.content_list.list.remove(file_path);
        self.save_content_list()?;
        Ok(())
    }
}
