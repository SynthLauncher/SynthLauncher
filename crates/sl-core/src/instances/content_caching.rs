use std::{collections::HashMap, fs::OpenOptions, path::Path};

use serde::{Deserialize, Serialize};

const MOD_LIST_FILE_NAME: &str = "mod_list.json";


#[derive(Debug, Deserialize, Serialize)]
pub enum ContentSource {
    Modrinth,
    Curseforge,
    External,
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

    pub async fn get_mod_list(&self) -> tokio::io::Result<ModList> {
        let path = self.instance_path.join(MOD_LIST_FILE_NAME);
        let data = tokio::fs::read(path).await?;
        Ok(serde_json::from_slice(&data).unwrap_or_else(|_| ModList::new()))
    }

    async fn save_mod_list(&self, new_mod_list: ModList) -> tokio::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.instance_path.join(MOD_LIST_FILE_NAME))?;
        serde_json::to_writer_pretty(file, &new_mod_list)?;

        Ok(())
    }

    pub async fn cache_mod(&self, filename: String, mod_data: ModData) -> tokio::io::Result<()> {
        let mut mod_list = self.get_mod_list().await?;
        mod_list.mods.insert(filename, mod_data);
        self.save_mod_list(mod_list).await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModData {
    pub name: String,
    pub hash: Option<String>,
    pub source: ContentSource,
}

impl ModData {
    pub fn new(name: String, hash: Option<String>, source: ContentSource) -> Self {
        Self { name, hash, source }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModList {
    scheme_version: u32,
    /// Key is the file name
    pub mods: HashMap<String, ModData>,
}

impl ModList {
    pub fn new() -> Self {
        Self {
            scheme_version: 0,
            mods: HashMap::new(),
        }
    }
}
