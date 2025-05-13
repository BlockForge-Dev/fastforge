use clap::{ Command};

pub fn build_cli() -> Command {
    Command::new("FastForge")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("FastForge: Smart AI-native installer for Move toolchains")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("login").about("Login to your FastForge account"))
        .subcommand(Command::new("logout").about("Logout from FastForge"))
        .subcommand(Command::new("install").about("Install toolchain locally or via cloud"))
        .subcommand(Command::new("upgrade").about("Upgrade to premium"))
        .subcommand(Command::new("refer").about("Generate/share your referral link"))
        .subcommand(Command::new("history").about("View install history"))
}
