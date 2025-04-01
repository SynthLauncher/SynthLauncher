use clap::Parser;
use cli::{Cli, Commands};
use synthlauncher_backend::{
    config::{app::init_launcher_dir, installations::{Installation, InstallationMetadata}},
    json::manifest::read,
};

mod cli;

#[tokio::main]
async fn main() {
    init_launcher_dir().await.unwrap();

    let cli = Cli::parse();

    match cli.command {
        Commands::Launch { name, version } => {
            let metadata = InstallationMetadata::new(name, version);
            let mut instance = Installation::new(metadata);
            let manifest = read();
            
            instance.install(&manifest).await.unwrap();
            instance.execute().unwrap();
        },
    }
}
