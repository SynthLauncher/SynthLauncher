use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use sl_core::launcher::instances::instance_metadata::ModLoader;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Default)]
pub struct LoaderInfo {
    /// Can be "vanilla"/"fabric"/"quilt"/"forge"
    pub loader: Option<ModLoader>,
    /// Depends on the loader, can be left empty for vanilla
    pub loader_version: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Imports an instance from a given path
    Import {
        path: PathBuf,
    },
    /// Exports an instance to a given path
    Export {
        instance_name: String,
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Creates a Minecraft instance
    Create {
        #[arg(required = true)]
        instance_name: String,
        #[arg(required = true)]
        version: String,
        #[command(flatten)]
        loader_info: LoaderInfo,
    },

    /// Launches a Minecraft instance
    Launch {
        #[arg(required = true)]
        instance_name: String,
    },

    /// Adds an offline player account
    AddOfflineAccount {
        #[arg(required = true)]
        name: String,
    },

    /// Adds a premium player account
    AddPremiumAccount,

    /// Sets the current account
    SetCurrentAccount {
        #[arg(required = true)]
        name: String,
    },

    /// Lists all player instances
    ListInstances,
    
    /// Lists all player accounts
    ListAccounts,
    
    /// Displays the current account
    CurrentAccount,
    
    /// Lists available Minecraft versions
    ListMinecraftVersions,
}
