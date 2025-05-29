use clap::Parser;
use cli::{Cli, Commands};
use sl_core::{
    config::{config::Config, init_launcher_dir},
    instance::{Instance, InstanceType},
    instances::Instances,
    profiles::{auth::AuthFlow, player::{PlayerProfile, PlayerProfiles}},
};
use sl_utils::utils::errors::BackendError;

mod cli;

#[tokio::main]
async fn main() -> Result<(), BackendError> {
    init_launcher_dir().await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Install {
            instance_name,
            version,
            loader_info,
        } => {
            let loader = loader_info.loader.unwrap_or_default();
            let loader_version = loader_info.loader_version.unwrap_or_default();

            let mut instance = Instance::new(&instance_name, &version, loader, None)?;

            match instance.instance_type {
                InstanceType::Vanilla => instance.install().await?,
                _ => {
                    instance.install().await?;
                    instance.install_loader(&loader_version).await?;
                }
            }
        }
        Commands::Launch {
            instance_name,
            username,
        } => {
            let mut config = Config::read_global().unwrap();
            config
                .update_config_field("auth_player_name", username.as_str())
                .unwrap();

            let instance = Instances::find(&instance_name)?;
            instance.execute(None).await.unwrap();
        }
        Commands::LaunchPremium { name } => {
            let mut config = Config::read_global().unwrap();
            let auth_access_token = &config.get("auth_access_token").unwrap().to_string();
            if auth_access_token == "0" {
                let mut auth = AuthFlow::new("74909cec-49b6-4fee-aa60-1b2a57ef72e1");
                let code_res = auth.request_code().await.unwrap();

                println!(
                "Open this link in your browser {} and enter the following code: {}\nWaiting authentication...",
                code_res.verification_uri, code_res.user_code
            );

                auth.wait_for_login().await.unwrap();
                auth.login_in_xbox_live().await.unwrap();
                let minecraft = auth.login_in_minecraft().await.unwrap();
                let profile = PlayerProfile::premium_account(minecraft.access_token.clone())
                    .await
                    .unwrap();
                config
                    .update_config_field("auth_access_token", minecraft.access_token.as_str())
                    .unwrap();

                let instance = Instances::find(&name).unwrap();
                instance.execute(Some(&profile)).await?;
            }

            let profile = PlayerProfile::premium_account(auth_access_token.to_string())
                .await
                .unwrap();
            let instance = Instances::find(&name).unwrap();
            instance.execute(Some(&profile)).await?;
        }
        Commands::RemoveInstallation { name } => {
            Instances::remove(&name)?;
        },
        Commands::AddOfflineProfile { name } => {
            let profile = PlayerProfile::offline_account(name).await?;
            PlayerProfiles::add(profile)?;
        }
    }

    Ok(())
}
