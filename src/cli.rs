use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        #[arg(required = true)]
        name: String,
        #[arg(required = true)]
        version: String,
    },
    Launch {
        #[arg(required = true)]
        name: String,
        #[arg(required = true)]
        username: String,
    },
    List,
}
