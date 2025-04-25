use clap::Parser;
use cli::{Cli, Commands};
use sl_core::{config::{config::Config, init_launcher_dir}, installations::{Installation, InstallationMetadata, Installations}, json::manifest::manifest_read};

mod cli;

#[tokio::main]
async fn main() {
    init_launcher_dir()
        .await
        .unwrap();

    let cli = Cli::parse();

    match cli.command {
        Commands::Install { name, version } => {
            let metadata = InstallationMetadata::new(name, version);
            let mut instance = Installation::new(metadata);
            let manifest = manifest_read();

            instance.install(&manifest).await.unwrap();
        }
        Commands::Launch { name, username } => {
            let mut config = Config::read_global().unwrap();
            config
                .update_config_field("auth_player_name", username.as_str())
                .unwrap();

            let installation = Installations::find(name).unwrap();
            installation.execute().unwrap();
        }
        Commands::List => {
            let installations = Installations::load();

            let mut count: i32 = 1;
            for installation in installations.0 {
                println!("{}: {}\n", count, installation.metadata.name());
                count += 1;
            }
        }
    }
}
