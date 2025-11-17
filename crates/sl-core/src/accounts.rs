use std::{collections::HashMap, io, path::Path};

use serde::{Deserialize, Serialize};
use sl_player::PlayerData;
use sl_utils::wlog;

/// Manages player accounts
pub struct AccountsManager<'a> {
    accounts_path: &'a Path,
}

impl<'a> AccountsManager<'a> {
    pub(crate) const fn new(accounts_path: &'a Path) -> Self {
        AccountsManager { accounts_path }
    }

    /// Load player accounts that belongs to a given manager
    pub async fn load(&self) -> io::Result<PlayerAccounts> {
        PlayerAccounts::load_from(self.accounts_path).await
    }

    /// Add a new player account to the manager under a given name
    pub async fn add_account(&mut self, name: String, data: PlayerData) -> io::Result<()> {
        add_account(self.accounts_path, name, data).await
    }

    /// Remove a player account from the manager under a given name
    pub async fn remove_account(&mut self, name: &str) -> io::Result<()> {
        remove_account(self.accounts_path, name).await
    }

    /// Set the current player account in the manager under a given name
    pub async fn set_current_account(&mut self, name: String) -> io::Result<()> {
        set_current_account(self.accounts_path, name).await
    }
}

type PlayerName = String;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerAccounts {
    scheme_version: u32,
    current_account: String,
    accounts: HashMap<PlayerName, PlayerData>,
}

impl PlayerAccounts {
    pub fn new() -> Self {
        PlayerAccounts {
            current_account: String::new(),
            accounts: HashMap::new(),
            scheme_version: 0,
        }
    }

    pub const fn accounts(&self) -> &HashMap<PlayerName, PlayerData> {
        &self.accounts
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

    /// Load player accounts from a given file path
    pub(crate) async fn load_from(path: &Path) -> io::Result<Self> {
        let content = match tokio::fs::read(path).await {
            Ok(content) => content,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Self::new()),
            Err(e) => return Err(e),
        };

        serde_json::from_slice(&content).or_else(|e| {
            wlog!(
                "failed to read player accounts data from '{}': {e}",
                path.display()
            );
            Ok(Self::new())
        })
    }

    /// Sync the given player accounts to their path
    pub(crate) async fn save_to(&self, path: &Path) -> std::io::Result<()> {
        let data = serde_json::to_vec_pretty(self)?;
        tokio::fs::write(path, data).await?;
        Ok(())
    }
}

/// Add a new player account to the given accounts list at path
pub(crate) async fn add_account(
    accounts_path: &Path,
    name: String,
    data: PlayerData,
) -> std::io::Result<()> {
    let mut accounts = PlayerAccounts::load_from(accounts_path).await?;
    accounts.current_account = name.clone();
    accounts.accounts.insert(name, data);
    accounts.save_to(accounts_path).await?;
    Ok(())
}

/// Remove a player account from the given accounts list at path
pub(crate) async fn remove_account(accounts_path: &Path, name: &str) -> std::io::Result<()> {
    let mut accounts = PlayerAccounts::load_from(accounts_path).await?;
    accounts.accounts.remove(name);

    if accounts.current_account == name {
        if let Some((new_current, _)) = accounts.accounts.iter().next() {
            accounts.current_account = new_current.clone();
        } else {
            accounts.current_account = String::new();
        }
    }

    accounts.save_to(accounts_path).await?;
    Ok(())
}

/// Set the current player account to the given name
pub(crate) async fn set_current_account(accounts_path: &Path, name: String) -> std::io::Result<()> {
    let mut accounts = PlayerAccounts::load_from(accounts_path).await?;

    if accounts.accounts.contains_key(&name) {
        accounts.current_account = name;
    }

    accounts.save_to(accounts_path).await?;
    Ok(())
}
