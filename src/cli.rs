use clap::{Parser, Subcommand};
use sl_core::instance::InstanceType;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a Minecraft instance
    Install {
        #[arg(required = true)]
        instance_name: String,
        #[arg(required = true)]
        version: String,

        #[arg(required = true)]
        loader: InstanceType,
        #[arg(required = true)]
        loader_version: String
    },
    /// Launch a Minecraft instance
    Launch {
        #[arg(required = true)]
        instance_name: String,
        #[arg(required = true)]
        username: String,
    },
    LaunchPremium {
        #[arg(required = true)]
        name: String,
    },
    RemoveInstallation {
        #[arg(required = true)]
        name: String
    }
}
