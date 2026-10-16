use std::{thread, time::Duration};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient, activity};

pub fn run()
{
    loop {
        let mut client = DiscordIpcClient::new("1369620733453664287");

        match client.connect()
        {
            Ok(()) => {
                loop
                {
                    let result = client.set_activity(
                        activity::Activity::new()
                        .state("In the launcher!")
                    );
    
                    if let Err(e) = result {
                        eprintln!("Discord RPC error: {e}");
                        break;
                    }
                    thread::sleep(Duration::from_secs(5));
                }
            }
            Err(e) => {
                eprintln!("Discord connection has failed: {e}");
            }
        }
    }
}