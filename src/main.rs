mod cli;
mod config;
mod commands;
mod services;
mod utils;
mod models;
mod constants;

use dotenv::dotenv;

fn main() {
    dotenv().ok(); // Load .env file at runtime
    let cli = cli::build_cli();
    let matches = cli.get_matches();
    cli::handle_matches(&matches);
}
