use std::{collections::HashMap, io, path::Path};

use serde::{Deserialize, Serialize};
use sl_player::PlayerAccount;
use sl_utils::{errors::BackendError, requester::Requester, wlog};

/// Manages player accounts
pub struct AccountsManager<'a> {
    accounts_path: &'a Path,
    accounts: PlayerAccounts,
}

impl<'a> AccountsManager<'a> {
    pub(crate) fn new(accounts_path: &'a Path) -> Self {
        AccountsManager {
            accounts_path,
            accounts: PlayerAccounts::new(),
        }
    }

    async fn load(&mut self) -> io::Result<()> {
        let accounts = PlayerAccounts::load_from(self.accounts_path).await?;
        self.accounts = accounts;
        Ok(())
    }

    async fn sync(&self) -> io::Result<()> {
        self.accounts.sync_to(self.accounts_path).await?;
        Ok(())
    }

    async fn refresh_current_account(&mut self, requester: &Requester) -> Result<(), BackendError> {
        self.load().await?;

        let PlayerAccounts {
            current_account,
            list,
            ..
        } = &mut self.accounts;

        if let Some(account) = list.get_mut(current_account) {
            account.refresh_account_token(&requester).await?;
        }

        self.sync().await?;
        Ok(())
    }

    /// Add a new player account to the manager under a given name
    pub async fn add_account(&mut self, player_acc: PlayerAccount) -> io::Result<()> {
        self.load().await?;

        self.accounts
            .list
            .insert(player_acc.username.clone(), player_acc);

        self.sync().await?;
        Ok(())
    }

    /// Remove a player account from the manager under a given name
    pub async fn remove_account(&mut self, name: &str) -> io::Result<()> {
        self.load().await?;

        self.accounts.list.remove(name);

        if name == self.accounts.current_account {
            if let Some((new_current, _)) = self.accounts.list.iter().next() {
                self.accounts.current_account = new_current.clone();
            } else {
                self.accounts.current_account = String::new();
            }
        }

        self.sync().await?;
        Ok(())
    }

    /// Set the current player account in the manager under a given name
    pub async fn set_current_account(&mut self, name: String) -> io::Result<()> {
        self.load().await?;
        if self.accounts.list.contains_key(&name) {
            self.accounts.current_account = name;
            self.sync().await?;
        }

        Ok(())
    }

    pub async fn get_current_account(&mut self) -> Result<Option<&PlayerAccount>, BackendError> {
        self.load().await?;
        Ok(self.accounts.get(&self.accounts.current_account))
    }
}

pub type PlayerAccountsList = HashMap<String, PlayerAccount>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerAccounts {
    scheme_version: u32,
    current_account: String,
    list: PlayerAccountsList,
}

impl PlayerAccounts {
    pub fn new() -> Self {
        PlayerAccounts {
            current_account: String::new(),
            list: PlayerAccountsList::new(),
            scheme_version: 0,
        }
    }

    pub const fn accounts(&self) -> &PlayerAccountsList {
        &self.list
    }

    pub fn get(&self, name: &str) -> Option<&PlayerAccount> {
        self.list.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut PlayerAccount> {
        self.list.get_mut(name)
    }

    /// Load player accounts that belongs to a given manager
    pub async fn load_from(accounts_path: &Path) -> io::Result<PlayerAccounts> {
        let content = match tokio::fs::read(accounts_path).await {
            Ok(content) => content,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(PlayerAccounts::new()),
            Err(e) => return Err(e),
        };

        serde_json::from_slice(&content).or_else(|e| {
            wlog!(
                "Failed to read player accounts data from '{}': {e}",
                accounts_path.display()
            );
            Ok(PlayerAccounts::new())
        })
    }

    /// Sync the given player accounts to their path
    pub async fn sync_to(&self, accounts_path: &Path) -> io::Result<()> {
        let data = serde_json::to_vec_pretty(&self)?;
        tokio::fs::write(accounts_path, data).await?;
        Ok(())
    }
}
