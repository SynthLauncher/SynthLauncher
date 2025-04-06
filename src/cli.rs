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
        #[arg(required = true)]
        name: String,
        #[arg(required = true)]
        version: String,
    },
}