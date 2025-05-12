use clap::{Arg, ArgMatches, Command};

use crate::commands::{
    login::handle_login,
    logout::handle_logout,
    install::handle_install,
    upgrade::handle_upgrade,
    refer::handle_refer,
    history::handle_history,
};

pub fn build_cli() -> Command {
    Command::new("fastforge")
        .version("0.1.0")
        .about("⚡ FastForge - The Dev Tooling Companion")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("login")
                .about("Authenticate with FastForge")
                .arg(
                    Arg::new("token")
                        .help("JWT or personal access token")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(Command::new("logout").about("Logout from FastForge"))
        .subcommand(Command::new("install").about("Install a new tool"))
        .subcommand(Command::new("upgrade").about("Upgrade your FastForge plan"))
        .subcommand(Command::new("refer").about("Generate or redeem a referral code"))
        .subcommand(Command::new("history").about("View installation history"))
}

pub fn handle_matches(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("login", sub_m)) => {
            let token = sub_m
                .get_one::<String>("token")
                .expect("Token required")
                .to_string();
            handle_login(token);
        }
        Some(("logout", _)) => {
            handle_logout();
        }
        Some(("install", _)) => {
            handle_install();
        }
        Some(("upgrade", _)) => {
            handle_upgrade();
        }
        Some(("refer", _)) => {
            handle_refer();
        }
        Some(("history", _)) => {
            handle_history();
        }
        _ => {
            eprintln!("Unknown command");
        }
    }
}
