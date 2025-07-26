use clap::Parser;
use cli::{Cli, Commands};
use sl_core::{
    launcher::{
        init_launcher_dir,
        instances::{
            self, instance_importer::import_instance_from_path, instance_metadata::InstanceMetadata,
        },
        player_accounts::{add_account, set_current_account, PlayerAccounts},
    },
    VERSION_MANIFEST,
};
use sl_player::PlayerData;
use sl_utils::{dlog, elog, errors::BackendError, log};
use tokio::io::{self};

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

            let _ = InstanceMetadata::create(instance_name, &version, loader, loader_version, None)
                .await?;
        }
        Commands::Launch { instance_name } => {
            let (instance, _) = instances::get_existing(&instance_name)?;
            dlog!("Instance found!");
            let loaded_instance = instance.load_init().await?;
            let (mut child, mut reader) = loaded_instance.execute().await?;
            let mut stdout = io::stdout();
            tokio::io::copy(&mut reader, &mut stdout).await?;

            if let Some(status) = child.try_wait()? {
                if status.success() {
                    dlog!("Instance exited successfully");
                } else {
                    dlog!("Instance exited with error code {:?}", status.code());
                }
            }
        }
        Commands::AddOfflineAccount { name } => {
            add_account(name, PlayerData::default())?;
        }
        // FIXME
        Commands::AddPremiumAccount => {
            // let mut profiles = PlayerProfiles::load()?;
            // let mut auth = AuthFlow::new("74909cec-49b6-4fee-aa60-1b2a57ef72e1");
            // let code_res = auth.request_code().await.unwrap();

            // log!("Open this link in your browser {} and enter the following code: {}\nWaiting authentication...", code_res.verification_uri, code_res.user_code);

            // auth.wait_for_login().await.unwrap();
            // auth.login_in_xbox_live().await.unwrap();
            // let minecraft = auth.login_in_minecraft().await.unwrap();
            // let profile = PlayerProfile::premium_account(minecraft.access_token.clone())
            //     .await
            //     .unwrap();
            // profiles.add(profile)?;
        }
        Commands::SetCurrentAccount { name } => {
            set_current_account(name)?;
        }
        Commands::ListInstances => {
            for (i, instance) in instances::get_all_instances()?.iter().enumerate() {
                println!("[{}] {:#?}", i, instance);
            }
        }
        Commands::ListAccounts => {
            for (i, profile) in PlayerAccounts::load()?.accounts.iter() {
                println!(
                    "[{}]: Access Token: {}; ID: {}",
                    i, profile.access_token, profile.id
                );
            }
        }
        Commands::CurrentAccount => {
            let accounts = PlayerAccounts::load()?;
            log!("Current Account: {}", accounts.get_current().0);
        }
        Commands::ListMinecraftVersions => {
            for version in VERSION_MANIFEST.versions() {
                println!("{}", version.id);
            }
        }
        Commands::Export {
            instance_name,
            output,
        } => {
            let (instance, _) = instances::get_existing(&instance_name)?;
            let exporter = instance.exporter_to_path(&output)?;
            exporter.export()?;
        }
        Commands::Import { path } => {
            import_instance_from_path(&path)?;
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
