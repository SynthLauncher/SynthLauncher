use std::{collections::HashMap, fs::OpenOptions};

use serde::{Deserialize, Serialize};
use sl_player::PlayerData;

use crate::PROFILES_PATH;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerAccounts {
    pub scheme_version: u32,
    pub current_account: String,
    pub accounts: HashMap<String, PlayerData>,
}

impl PlayerAccounts {
    pub fn new() -> Self {
        PlayerAccounts {
            current_account: String::new(),
            accounts: HashMap::new(),
            scheme_version: 0
        }
    }

    pub fn get(&self, name: &str) -> &PlayerData {
        self.accounts.get(name).expect("Account must exist!")
    }

    pub fn get_current(&self) -> (&str, &PlayerData) {
        let name = &self.current_account;
        let data = self
            .accounts
            .get(name)
            .expect("Current account must exist!");
        (name.as_str(), data)
    }

    pub fn load() -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&PROFILES_PATH.as_path())?;
        serde_json::from_str(&content).or_else(|_| Ok(Self::new()))
    }

    pub fn save(new_accounts: &PlayerAccounts) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(PROFILES_PATH.as_path())?;

        serde_json::to_writer_pretty(file, &new_accounts)?;

        Ok(())
    }
}

pub fn add_account(name: String, data: PlayerData) -> std::io::Result<()> {
    let mut accounts = PlayerAccounts::load()?;
    accounts.current_account = name.clone();
    accounts.accounts.insert(name, data);
    PlayerAccounts::save(&accounts)?;
    
    Ok(())
}

pub fn remove_account(name: &str) -> std::io::Result<()> {
    let mut accounts = PlayerAccounts::load()?;
    accounts.accounts.remove(name);

    if accounts.current_account == name {
        if let Some((new_current, _)) = accounts.accounts.iter().next() {
            accounts.current_account = new_current.clone();
        } else {
            accounts.current_account = String::new();
        }
    }

    PlayerAccounts::save(&accounts)?;

    Ok(())
}

pub fn set_current_account(name: String) -> std::io::Result<()> {
    let mut accounts = PlayerAccounts::load()?;
    if accounts.accounts.contains_key(&name) {
        accounts.current_account = name;
    }

    PlayerAccounts::save(&accounts)?;
    
    Ok(())
}
