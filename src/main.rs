use clap::Parser;
use cli::{Cli, Commands};
use sl_core::environment::LauncherEnv;
use sl_player::auth::ms_auth::{login_1_link, login_2_wait, login_3_xbox};
use sl_utils::{dlog, elog, errors::BackendError, log, progress::ProgressReceiver};
use tokio::io::{self};

mod cli;

async fn run_cli() -> Result<(), BackendError> {
    let env = LauncherEnv::new_at_default();
    let requester = env.requester();
    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            instance_name,
            version,
            loader_info,
        } => {
            let loader = loader_info.loader.unwrap_or_default();
            let loader_version = loader_info.loader_version;

            env.instances()
                .create_instance(instance_name, &version, loader, loader_version)
                .await?;
        }
        Commands::Launch { instance_name } => {
            let instances = env.instances();
            let (instance, _) = instances.get_existing(&instance_name)?;

            dlog!("Instance found!");
            let loaded_instance = instance
                .load_init(&instances, ProgressReceiver::new(|_| {}))
                .await?;
            let (mut child, mut reader) = loaded_instance.execute().await?;
            let mut stdout = io::stdout();
            loop {
                tokio::io::copy(&mut reader, &mut stdout).await?;

                if let Some(status) = child.try_wait()? {
                    if status.success() {
                        dlog!("Instance exited successfully");
                    } else {
                        dlog!("Instance exited with error code {:?}", status.code());
                    }

                    break;
                }
            }
        }
        Commands::AddOfflineAccount { name } => {
            let mut accounts = env.accounts();
        }
        Commands::AddPremiumAccount => {
            let auth_code_res = login_1_link(requester).await?;
            let auth_token_res = login_2_wait(auth_code_res, requester).await?;
            let account = login_3_xbox(auth_token_res, true, requester).await?;
            env.accounts().add_account(account).await?;
        }
        Commands::SetCurrentAccount { name } => {
            let mut accounts = env.accounts();
            accounts.set_current_account(name).await?;
        }
        Commands::ListInstances => {
            let instances = env.instances();
            for (i, instance) in instances.get_all_instances().await?.iter().enumerate() {
                println!("[{}] {:#?}", i, instance);
            }
        }
        Commands::ListAccounts => {
            let accounts = env.accounts();
            // let accounts = accounts.load().await?;

            // for (i, profile) in accounts.accounts().iter() {
            //     println!(
            //         "[{}]: Access Token: {}; ID: {}",
            //         i, profile.access_token, profile.id
            //     );
            // }
        }
        Commands::CurrentAccount => {
            let accounts = env.accounts();
            // let accounts = accounts.load().await?;

            // let (current_account, _) = accounts.get_current();
            // log!("Current Account: {}", current_account);
        }
        Commands::ListMinecraftVersions => {
            let manifest = env.version_manifest().await;

            for version in manifest.versions() {
                println!("{}", version.id);
            }
        }
        Commands::Export {
            instance_name,
            output,
        } => {
            let instances = env.instances();
            let (instance, _) = instances.get_existing(&instance_name)?;
            let exporter = instance.exporter_to_path(&instances, &output)?;
            exporter.export()?;
        }
        Commands::Import { path } => {
            let mut instances = env.instances();
            instances.import_instance_from_path(&path).await?
        }
        Commands::Test => todo!(),
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
