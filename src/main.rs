use clap::Parser;
use cli::{Cli, Commands};
use discord_rpc_client::Client;
use sl_core::{
    auth::{AuthFlow, PlayerProfile}, config::{config::Config, init_launcher_dir}, installations::{Installation, Installations}
};
use sl_mod_manager::modrinth::install_modrinth_file;
use sl_utils::utils::errors::BackendError;

mod cli;

#[tokio::main]
async fn main() -> Result<(), BackendError> {
    init_launcher_dir().await.unwrap();

    let cli = Cli::parse();

    match cli.command {
        Commands::Install { name, version } => {
            let mut instance = Installation::new(&name, &version).unwrap();
            instance.install().await.unwrap();
        }
        Commands::Launch { name, username } => {
            let mut config = Config::read_global().unwrap();
            config
                .update_config_field("auth_player_name", username.as_str())
                .unwrap();

            let rpc_handle = tokio::spawn(async {
                let mut drpc = Client::new(1369620733453664287);
                drpc.start();

                let start_time = chrono::Utc::now().timestamp(); 

                loop {
                    let _ = drpc.set_activity(|a| {
                        a.state("Playing Minecraft")
                            .details("In the launcher")
                            .timestamps(|t| t.start(start_time as u64))
                            .assets(|act| act
                                .large_image("logo")
                                .large_text("Minecraft"))
                    });
            
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                } 
            });

            let instance = Installations::find(&name).unwrap();
            instance.execute(None).unwrap();
            
            rpc_handle.abort();
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
            instance.install().await.unwrap();
        }
        Commands::List => {
            let installations = Installations::load()?;
            let mut count: i32 = 1;
            for installation in installations.0 {
                println!("{}: {}", count, installation.name);
                count += 1;
            }
        },
        Commands::LaunchPremium { name } => {
            let mut auth = AuthFlow::new("74909cec-49b6-4fee-aa60-1b2a57ef72e1");
            let code_res = auth.request_code().await.unwrap();
        
            println!(
                "Open this link in your browser {} and enter the following code: {}\nWaiting authentication...",
                code_res.verification_uri, code_res.user_code
            );
            
            auth.wait_for_login().await.unwrap();
            auth.login_in_xbox_live().await.unwrap();
            let minecraft = auth.login_in_minecraft().await.unwrap();
            let profile = PlayerProfile::new(minecraft.access_token.clone()).await.unwrap();

            let instance = Installations::find(&name).unwrap();
            instance.execute(Some(&profile)).unwrap();
        },
        Commands::AddMod { name, id } => {
            let installation = Installations::find(&name).unwrap();
            let dest = installation.dir_path().join("mods");

            println!("{:?}", dest);
            install_modrinth_file(&id, &dest).await.unwrap();
        },
        Commands::RemoveInstallation { name } => {
            Installations::remove(&name)?;
        }
    }

    Ok(())
}

