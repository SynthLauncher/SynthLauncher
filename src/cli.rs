use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Launch {
        /// The version to launch
        #[arg(required = true)]
        version: String,
        /// The player name
        #[arg(default_value = "Player")]
        name: String,
    },
}