use clap::{Parser, Subcommand};
use config::{Config, File};
use home::home_dir;
use std::path::{Path, PathBuf};

#[derive(Parser, Clone)]
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
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    /// Generates new feature module
    GenFeat {
        /// The name of the new feature
        #[arg(short, long)]
        feature: String,
    },
    /// Generates new sub-feature for a feature module
    GenSubfeat {
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
            File::from(get_global_config_path())
                .format(config::FileFormat::Toml)
                .required(false),
        )
        .add_source(
            File::with_name(".feature_generator_config.toml")
                .format(config::FileFormat::Toml)
                .required(false),
        )
        .build();

    if let Ok(conf) = config {
        let base_package = conf.get_string("base-package");
        let app_name = conf.get_string("app-name");
        args.app_name = args.app_name.or(app_name.ok());

        args.base_package = args.base_package.or(base_package.ok());
    }

    let is_config = matches!(&args.command, Command::Config { .. });
    if !is_config {
        args.base_package.as_ref().expect(
            "\
    No base-package found either in arguments or config files. Please consider adding base-package \
    feature_generator config --base-package <base_package>",
        );

        args.app_name.as_ref().expect(
            "\
    No app-name found either in arguments or config files. Please run: \
    feature_generator config --app-name <your_app_name>",
        );
    }

    args
}

pub fn get_global_config_path() -> PathBuf {
    home_dir()
        .unwrap()
        .join(".config")
        .join(".feature_generator")
        .join("config.toml")
}

pub fn get_local_config_path() -> PathBuf {
    let path = Path::new(".feature_generator_config.toml");
    path.to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn todo() {}
}
