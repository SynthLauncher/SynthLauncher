use clap::{Parser, Subcommand};

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
        name: String,
        #[arg(required = true)]
        version: String,
    },
    /// Install Fabric for a Minecraft instance
    InstallFabric {
        /// Name of the Minecraft instance to install Fabric for
        #[arg(required = true)]
        instance_name: String,
        /// Version of Fabric to install
        #[arg(required = true)]
        loader_version: String,
    },
    /// Launch a Minecraft instance
    Launch {
        #[arg(required = true)]
        name: String,
        #[arg(required = true)]
        username: String,
    },
    /// List all installed Minecraft instances
    List,
    LaunchPremium {
        #[arg(required = true)]
        name: String,
    },
    AddMod {
        #[arg(required = true)]
        name: String,
        #[arg(required = true)]
        id: String
    },
    RemoveInstallation {
        #[arg(required = true)]
        name: String
    }
}
