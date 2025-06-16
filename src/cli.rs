use clap::{ArgAction, Args, Parser, Subcommand};
use sl_core::launcher::instance::InstanceType;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Default)]
pub struct LoaderInfo {
    /// Can be "vanilla" or "fabric" or "quilt" or "forge"
    pub loader: Option<InstanceType>,
    /// Depends on the loader, can be left empty for vanilla
    pub loader_version: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Installs a Minecraft instance
    Install {
        #[arg(required = true)]
        instance_name: String,
        #[arg(required = true)]
        version: String,
        #[command(flatten)]
        loader_info: LoaderInfo
    },
    /// Launch a Minecraft instance
    Launch {
        #[arg(required = true)]
        instance_name: String,
    },
    AddOfflineProfile {
        #[arg(required = true)]
        name: String,
    },
    AddPremiumProfile,
    SetCurrentProfile {
        #[arg(required = true)]
        name: String,
        #[arg(long, action = ArgAction::SetTrue)]
        premium: bool,
    },

}
