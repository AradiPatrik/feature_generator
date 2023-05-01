use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Feature Generator")]
#[command(author = "Aradi Patrik <aradipatrik@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Generates new features and screens inside them", long_about = None)]
pub struct Cli {
    /// Turn debugging on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    /// base package name
    #[arg(short, long)]
    pub base_package: String,

    /// Application name
    #[arg(short, long)]
    pub app_name: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generates new feature module
    GenMod {
        /// The name of the new feature
        #[arg(short, long)]
        feature: String,

        /// The name of the starting screen
        #[arg(short, long)]
        start_screen: String
    },
    /// Generates new screen for a feature module
    GenScreen {
        /// The name of the existing feature
        #[arg(short, long)]
        feature: String,

        /// The name of the new screen
        #[arg(short, long)]
        screen: String
    }
}

pub fn parse_args() -> Cli {
    return Cli::parse()
}
