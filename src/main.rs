use clap::Parser;
use cli::{Cli, Commands};
use sl_core::{
    config::init_launcher_dir,
    instance::{Instance, InstanceType},
    instances::Instances,
    profiles::{
        auth::AuthFlow,
        player::{PlayerProfile, PlayerProfiles},
    }, 
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
        Commands::Launch { instance_name } => {
            let profiles = PlayerProfiles::load()?;
            let current_profile = profiles.current_profile().unwrap();

            let instance = Instances::find(&instance_name)?;
            instance.execute(current_profile).await.unwrap();
        },
        Commands::AddOfflineProfile { name } => {
            let mut profiles = PlayerProfiles::load()?;
            let profile = PlayerProfile::offline_account(name).await?;
            profiles.add(profile)?;
        }
        Commands::AddPremiumProfile => {
            let mut profiles = PlayerProfiles::load()?;
            let mut auth = AuthFlow::new("74909cec-49b6-4fee-aa60-1b2a57ef72e1");
            let code_res = auth.request_code().await.unwrap();

            println!("Open this link in your browser {} and enter the following code: {}\nWaiting authentication...", code_res.verification_uri, code_res.user_code);

            auth.wait_for_login().await.unwrap();
            auth.login_in_xbox_live().await.unwrap();
            let minecraft = auth.login_in_minecraft().await.unwrap();
            let profile = PlayerProfile::premium_account(minecraft.access_token.clone())
                .await
                .unwrap();
            profiles.add(profile)?;
        },
        Commands::SetCurrentProfile { name, premium } => {
            let mut profiles = PlayerProfiles::load()?;
            let (profile, index) = profiles.find(&name, premium)?;
            if let Some(_) = profile {
                profiles.set_current_profile(index)?;
            } else {
                return Ok(());
            }
        }
    }

    Ok(())
}
