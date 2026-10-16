// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

mod discord_rpc;

fn main() {
    if let Err(e) =  thread::Builder::new()
    .name("discord-rpc".into())
    .spawn(|| {
        discord_rpc::run()
    })
    {
        eprintln!("Failed to spawn Discord RPC thread: {e}");
    }

    sl_frontend_lib::run()
}
