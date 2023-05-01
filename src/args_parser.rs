use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use config::{Config, File};
use home::home_dir;

#[derive(Parser)]
#[command(name = "Feature Generator")]
#[command(author = "Aradi Patrik <aradipatrik@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Generates new features and screens inside them", long_about = None)]
pub struct Cli {
    /// Turn debugging on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    /// Optional Base package name
    #[arg(short, long)]
    pub base_package: Option<String>,

    /// Optional Application name
    #[arg(short, long)]
    pub app_name: Option<String>,

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
        start_screen: String,
    },
    /// Generates new screen for a feature module
    GenScreen {
        /// The name of the existing feature
        #[arg(short, long)]
        feature: String,

        /// The name of the new screen
        #[arg(short, long)]
        screen: String,
    },
    /// Adds local or global configuration
    Config {
        /// Configure globally
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        global: bool,

        /// Base package name
        #[arg(short, long)]
        base_package: Option<String>,

        /// Application name
        #[arg(short, long)]
        app_name: Option<String>,
    },
}

pub fn parse_args() -> Cli {
    let mut args = Cli::parse();
    let config = Config::builder()
        .add_source(
            File::from(
                get_global_config_path()
            ).format(config::FileFormat::Toml)
                .required(false)
        )
        .add_source(
            File::with_name(".feature_generator_config.toml")
                .format(config::FileFormat::Toml)
                .required(false)
        )
        .build();

    if let Ok(conf) = config {
        let base_package = conf.get_string("base-package");
        let app_name = conf.get_string("app-name");
        args.app_name = args.app_name.or(app_name.ok());
        args.base_package = args.base_package.or(base_package.ok());
    }

    let is_config = matches!(&args.command, Commands::Config { .. });
    if !is_config {
        args.base_package.as_ref().expect("\
    No base-package found either in arguments or config files. Please consider adding base-package \
    feature_generator config --base-package <base_package>");

        args.app_name.as_ref().expect("\
    No app-name found either in arguments or config files. Please run: \
    feature_generator config --app-name <your_app_name>");
    }

    args
}

pub fn get_global_config_path() -> PathBuf {
    let path = home_dir().unwrap().join(".config")
        .join(".feature_generator")
        .join("config.toml");
    path
}

pub fn get_local_config_path() -> PathBuf {
    let path = Path::new(".feature_generator_config.toml");
    path.to_owned()
}
