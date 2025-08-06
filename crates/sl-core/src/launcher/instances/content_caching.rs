use std::{collections::HashMap, fs::OpenOptions, path::Path};

use serde::{Deserialize, Serialize};

pub const MOD_LIST_FILE_NAME: &str = "modlist.json";

#[derive(Debug, Deserialize, Serialize)]
pub enum ContentSource {
    Modrinth,
    Curseforge,
    External,
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

type Filename = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct ModList {
    pub scheme_version: u32,
    pub mods: HashMap<Filename, ModData>,
}

impl ModList {
    pub fn new() -> Self {
        Self {
            scheme_version: 0,
            mods: HashMap::new(),
        }
    }

    pub async fn load(instance_path: &Path) -> std::io::Result<Self> {
        let path = instance_path.join(MOD_LIST_FILE_NAME);
        let data = tokio::fs::read(&path).await?;
        Ok(serde_json::from_slice(&data).unwrap_or_else(|_| Self::new()))
    }

    pub fn save(instance_path: &Path, new_mod_list: &ModList) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(instance_path.join(MOD_LIST_FILE_NAME))?;

        serde_json::to_writer_pretty(file, &new_mod_list)?;

        Ok(())
    }
}

pub async fn cache_mod_for_instance(
    instance_path: &Path,
    filename: String,
    mod_data: ModData,
) -> std::io::Result<()> {
    let mut mod_list = ModList::load(&instance_path).await?;
    mod_list.mods.insert(filename, mod_data);
    ModList::save(&instance_path, &mod_list)?;

    Ok(())
}
