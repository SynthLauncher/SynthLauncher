use clap::Parser;
use cli::{Cli, Commands};
use sl_core::{
    config::{config::Config, init_launcher_dir},
    installations::{Installation, Installations}
};
mod cli;

#[tokio::main]
async fn main() {
    init_launcher_dir().await.unwrap();

    let cli = Cli::parse();

    match cli.command {
        Commands::Install { name, version } => {
            let mut instance = Installation::new(name, version).unwrap();

            instance.install().await.unwrap();
        }
        Commands::Launch { name, username } => {
            let mut config = Config::read_global().unwrap();
            config
                .update_config_field("auth_player_name", username.as_str())
                .unwrap();

            let instance = Installations::find(&name).unwrap();
            instance.execute().unwrap()
        }
        Commands::InstallFabric {
            instance_name,
            loader_version,
        } => {
            let mut instance =
                Installations::find(&instance_name).expect("failed to find instance");
            instance
                .install_fabric(&loader_version)
                .await
                .expect("failed to install fabric");
            // has to reinstall probably it should reinstall every execute so if something went wrong.........
            instance.install().await.unwrap();
        }
        Commands::List => {
            let installations = Installations::load();

            let mut count: i32 = 1;
            for installation in installations.0 {
                println!("{}: {}", count, installation.name);
                count += 1;
            }
        }
    }
}
