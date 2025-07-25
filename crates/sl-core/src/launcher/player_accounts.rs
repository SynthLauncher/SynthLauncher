use std::{collections::HashMap, fs::OpenOptions};

use serde::{Deserialize, Serialize};
use sl_player::PlayerData;

use crate::PROFILES_PATH;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerAccounts {
    current_account: String,
    pub accounts: HashMap<String, PlayerData>,
}

impl PlayerAccounts {
    pub fn new() -> Self {
        let default = PlayerData::default();

        let mut accounts = HashMap::new();
        accounts.insert("synther".to_string(), default);

        PlayerAccounts {
            current_account: "synther".to_string(),
            accounts,
        }
    }

    pub fn get(&self, name: &str) -> &PlayerData {
        self.accounts.get(name).expect("Account must exist!")
    }

    pub fn get_current(&self) -> (&str, &PlayerData) {
        let name = &self.current_account;
        let data = self.accounts.get(name).expect("Current account must exist!");
        (name.as_str(), data)
    }

    pub fn set_current(&mut self, name: String) -> std::io::Result<()> {
        if self.accounts.contains_key(&name) {
            self.current_account = name;
        }

        Self::overwrite(&self)?;

        Ok(())
    }

    pub fn load() -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&PROFILES_PATH.as_path())?;
        serde_json::from_str(&content).or_else(|_| Ok(Self::new()))
    }

    pub fn overwrite(new_accounts: &PlayerAccounts) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(PROFILES_PATH.as_path())?;

        serde_json::to_writer_pretty(file, &new_accounts)?;
        
        Ok(())
    }

    pub fn add(&mut self, name: String, data: PlayerData) -> std::io::Result<()> {
        self.accounts.insert(name, data);
        Self::overwrite(&self)?;
        Ok(())
    }
    
    pub fn remove(&mut self, name: &str) -> std::io::Result<()> {
        self.accounts.remove(name);
        Self::overwrite(&self)?;
        Ok(())
    }
}
