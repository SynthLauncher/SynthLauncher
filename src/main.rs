use clap::Parser;
use cli::{Cli, Commands};
use sl_core::{
    launcher::{
        init_launcher_dir,
        instances::{self, metadata::InstanceMetadata},
        player::{
            microsoft_auth::AuthFlow, player_profile::PlayerProfile,
            player_profiles::PlayerProfiles,
        },
    },
    VERSION_MANIFEST,
};
use sl_utils::{dlog, elog, errors::BackendError, log};

mod cli;

async fn run_cli() -> Result<(), BackendError> {
    init_launcher_dir().await?;

    dlog!("Launcher initialized!");

    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            instance_name,
            version,
            loader_info,
        } => {
            let loader = loader_info.loader.unwrap_or_default();
            let loader_version = loader_info.loader_version;

            let _ =
                InstanceMetadata::create(&instance_name, &version, loader, loader_version, None)
                    .await?;
        }
        Commands::Launch { instance_name } => {
            let (instance, _) = instances::get_existing(&instance_name)?;
            dlog!("Instance found!");
            let loaded_instance = instance.load_init().await?;
            loaded_instance.execute().await?;
        }
        Commands::AddOfflineProfile { name } => {
            let mut profiles = PlayerProfiles::load()?;
            let profile = PlayerProfile::offline_account(name).await?;
            profiles.add(profile)?;
        }
        Commands::AddPremiumProfile => {
            let mut profiles = PlayerProfiles::load()?;
            let mut auth = AuthFlow::new("74909cec-49b6-4fee-aa60-1b2a57ef72e1");
            let code_res = auth.request_code().await.unwrap();

            log!("Open this link in your browser {} and enter the following code: {}\nWaiting authentication...", code_res.verification_uri, code_res.user_code);

            auth.wait_for_login().await.unwrap();
            auth.login_in_xbox_live().await.unwrap();
            let minecraft = auth.login_in_minecraft().await.unwrap();
            let profile = PlayerProfile::premium_account(minecraft.access_token.clone())
                .await
                .unwrap();
            profiles.add(profile)?;
        }
        Commands::SetCurrentProfile { name, premium } => {
            let mut profiles = PlayerProfiles::load()?;
            let (profile, index) = profiles.find(&name, premium)?;
            if let Some(_) = profile {
                profiles.set_current_profile(index)?;
            } else {
                return Ok(());
            }
        }
        Commands::ListInstances => {
            for (i, instance) in instances::get_all_instances()?.iter().enumerate() {
                println!("[{}] {:#?}", i, instance);
            }
        }
        Commands::ListProfiles => {
            for (i, profile) in PlayerProfiles::load()?.profiles.iter().enumerate() {
                println!("[{}] {:#?}", i, profile);
            }
        }
        Commands::CurrentProfile => {
            let profiles = PlayerProfiles::load()?;
            let profile = profiles.current_profile();
            println!("{:#?}", profile.as_ref())
        }
        Commands::ListMinecraftVersions => {
            for version in VERSION_MANIFEST.versions() {
                println!("{}", version.id);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    if let Err(err) = run_cli().await {
        elog!("{err}");
        return Err(());
    }
    Ok(())
}
