mod cli;
mod config;
mod constants;
mod services;
mod commands;
mod utils;
mod models;

use crate::cli::build_cli;
use tracing_subscriber;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load CLI and parse arguments
    let matches = build_cli().get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("login", _)) => {
            if let Err(e) = commands::login::handle_login().await {
                eprintln!("❌ Login failed: {}", e);
            }
        }
        Some(("install", _sub_m)) => {
            commands::install::handle_install().await.unwrap();
        }
        Some(("logout", _)) => {
            println!("🚪 Logout command placeholder (Day 4)");
        }
        _ => {
            println!("🛠 Use `--help` to see available commands.");
        }
    }

    Ok(())
}
