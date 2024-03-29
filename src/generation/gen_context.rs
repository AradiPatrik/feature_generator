use std::{
    path::{Path, PathBuf},
    process::exit,
};

use convert_case::{Case, Casing};

use crate::args_parser::{
    Cli,
    Command::{self},
};

#[derive(Debug, PartialEq)]
pub enum GenCtx {
    App(AppGenCtx),
    Config(Config),
}

impl GenCtx {
    pub fn app_ctx(&self) -> Option<&AppCtx> {
        match self {
            GenCtx::App(app) => Some(app.app_ctx()),
            GenCtx::Config(_) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AppGenCtx {
    Feature(FeatureGenCtx),
    Library(Library),
}

impl AppGenCtx {
    fn app_ctx(&self) -> &AppCtx {
        match self {
            AppGenCtx::Feature(FeatureGenCtx::Root(feature)) => &feature.app_context,
            AppGenCtx::Feature(FeatureGenCtx::Subfeature(subfeature)) => &subfeature.app_context,
            AppGenCtx::Library(library) => &library.app_context,
        }
    }

    pub fn module_name(&self) -> &str {
        match self {
            AppGenCtx::Feature(FeatureGenCtx::Root(feature)) => &feature.feature_name,
            AppGenCtx::Feature(FeatureGenCtx::Subfeature(subfeature)) => &subfeature.feature_name,
            AppGenCtx::Library(library) => &library.library_name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FeatureGenCtx {
    Root(Feature),
    Subfeature(Subfeature),
}

impl FeatureGenCtx {
    pub fn feature_name(&self) -> &str {
        match self {
            FeatureGenCtx::Root(r) => &r.feature_name,
            FeatureGenCtx::Subfeature(s) => &s.feature_name,
        }
    }

    pub fn impl_package_path(&self) -> Box<Path> {
        let (app_ctx, feature_name) = match self {
            FeatureGenCtx::Root(ref root) => (&root.app_context, &root.feature_name),
            FeatureGenCtx::Subfeature(ref subfeature) => {
                (&subfeature.app_context, &subfeature.feature_name)
            }
        };

        app_ctx.feature_impl_package_path(feature_name)
    }

    pub fn api_package_path(&self) -> Box<Path> {
        let (app_ctx, feature_name) = match self {
            FeatureGenCtx::Root(ref root) => (&root.app_context, &root.feature_name),
            FeatureGenCtx::Subfeature(ref subfeature) => {
                (&subfeature.app_context, &subfeature.feature_name)
            }
        };

        app_ctx.feature_api_package_path(feature_name)
    }
}

#[derive(Debug, PartialEq)]
pub enum CtxCreationError {
    AppNameMissing,
    BasePackageNameMissing,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AppCtx {
    pub is_testing: bool,
    pub app_name: String,
    pub base_package: String,
}

impl AppCtx {
    pub fn feature_path(&self, feature_name: &str) -> Box<Path> {
        let mut path = PathBuf::default();
        path.push("feature");
        path.push(feature_name.to_string().to_case(Case::Kebab));
        path.into()
    }

    pub fn feature_impl_path(&self, feature_name: &str) -> Box<Path> {
        let mut path = PathBuf::from(self.feature_path(feature_name));
        path.push("impl");

        path.into()
    }

    pub fn feature_impl_package_path(&self, feature_name: &str) -> Box<Path> {
        let mut path = PathBuf::from(self.feature_impl_path(feature_name));
        path.push("src/main/kotlin");
        path.push(self.base_package_path_part());

        path.into()
    }

    pub fn feature_api_path(&self, feature_name: &str) -> Box<Path> {
        let mut path = PathBuf::from(self.feature_path(feature_name));
        path.push("api");

        path.into()
    }

    pub fn feature_api_package_path(&self, feature_name: &str) -> Box<Path> {
        let mut path = PathBuf::from(self.feature_api_path(feature_name));
        path.push("src/main/kotlin");
        path.push(self.base_package_path_part());

        path.into()
    }

    pub fn base_package_path_part(&self) -> Box<Path> {
        Path::new(&self.base_package.replace('.', "/")).into()
    }
}

#[derive(Debug, PartialEq)]
pub struct Feature {
    pub app_context: AppCtx,
    pub feature_name: String,
}

#[derive(Debug, PartialEq)]
pub struct Subfeature {
    pub app_context: AppCtx,
    pub feature_name: String,
    pub subfeature_name: String,
}

#[derive(Debug, PartialEq)]
pub struct Library {
    pub app_context: AppCtx,
    pub library_name: String,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub app_name: Option<String>,
    pub base_package_name: Option<String>,
    pub global: bool,
}

impl From<Feature> for GenCtx {
    fn from(val: Feature) -> Self {
        GenCtx::feature_context(val)
    }
}

impl From<Subfeature> for GenCtx {
    fn from(val: Subfeature) -> Self {
        GenCtx::subfeature_context(val)
    }
}

impl From<Library> for GenCtx {
    fn from(val: Library) -> Self {
        GenCtx::library_context(val)
    }
}

impl From<Config> for GenCtx {
    fn from(val: Config) -> Self {
        GenCtx::config_context(val)
    }
}

impl GenCtx {
    fn feature_context(feature: Feature) -> Self {
        GenCtx::App(AppGenCtx::Feature(FeatureGenCtx::Root(feature)))
    }

    fn subfeature_context(subfeature: Subfeature) -> Self {
        GenCtx::App(AppGenCtx::Feature(FeatureGenCtx::Subfeature(subfeature)))
    }

    fn library_context(library: Library) -> Self {
        GenCtx::App(AppGenCtx::Library(library))
    }

    fn config_context(config: Config) -> Self {
        GenCtx::Config(config)
    }
}

impl GenCtx {
    pub fn from_cli(cli: Cli) -> Result<Self, CtxCreationError> {
        let app_context = AppCtx {
            app_name: cli.app_name.ok_or(CtxCreationError::AppNameMissing)?,
            base_package: cli
                .base_package
                .ok_or(CtxCreationError::BasePackageNameMissing)?,
            is_testing: cli.debug,
        };

        match cli.command {
            Command::GenerateCompletion { shell: _ } => exit(-1),
            Command::GenFeat { feature } => {
                let feature = Feature {
                    app_context,
                    feature_name: feature,
                };
                Ok(GenCtx::from(feature))
            }
            Command::GenSubfeat { feature, screen } => {
                let subfeature = Subfeature {
                    app_context,
                    feature_name: feature,
                    subfeature_name: screen,
                };
                Ok(GenCtx::from(subfeature))
            }
            Command::GenLib { lib } => {
                let library = Library {
                    app_context,
                    library_name: lib,
                };
                Ok(GenCtx::from(library))
            }
            Command::Config {
                global,
                base_package,
                app_name,
            } => {
                let config = Config {
                    app_name,
                    base_package_name: base_package,
                    global,
                };
                Ok(GenCtx::from(config))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args_parser::{Cli, Command};

    #[test]
    fn app_name_missing() {
        let cli = Cli {
            debug: false,
            base_package: Some("test.base.package".into()),
            app_name: None,
            command: Command::GenFeat {
                feature: "test_feature".into(),
            },
        };

        assert_eq!(Err(CtxCreationError::AppNameMissing), GenCtx::from_cli(cli))
    }

    #[test]
    fn base_package_missing() {
        let cli = Cli {
            debug: false,
            base_package: None,
            app_name: Some("test_app_name".into()),
            command: Command::GenFeat {
                feature: "test_feature".into(),
            },
        };

        assert_eq!(
            Err(CtxCreationError::BasePackageNameMissing),
            GenCtx::from_cli(cli),
        )
    }

    #[test]
    fn generate_feature() {
        let cli = Cli::with_command(Command::GenFeat {
            feature: "test_feature".into(),
        });

        assert_eq!(
            Ok(Feature {
                app_context: AppCtx::default(),
                feature_name: "test_feature".into(),
            }
            .into()),
            GenCtx::from_cli(cli)
        );
    }

    #[test]
    fn generate_subfeature() {
        let cli = Cli::with_command(Command::GenSubfeat {
            feature: "test_feature".into(),
            screen: "test_subfeature".into(),
        });

        assert_eq!(
            Ok(Subfeature {
                app_context: AppCtx::default(),
                feature_name: "test_feature".into(),
                subfeature_name: "test_subfeature".into(),
            }
            .into()),
            GenCtx::from_cli(cli)
        )
    }

    #[test]
    fn gen_config() {
        let cli = Cli::with_command(Command::Config {
            global: true,
            base_package: Some("test.base.package".into()),
            app_name: Some("test_app_name".into()),
        });

        assert_eq!(
            Ok(Config {
                app_name: Some("test_app_name".into()),
                base_package_name: Some("test.base.package".into()),
                global: true
            }
            .into()),
            GenCtx::from_cli(cli)
        );
    }

    #[test]
    fn feature_path() {
        let app_ctx = AppCtx::default();

        assert_eq!(
            Path::new("feature/test-feature"),
            app_ctx.feature_path("test-feature").as_ref()
        )
    }

    #[test]
    fn feature_impl_path() {
        let app_ctx = AppCtx::default();

        assert_eq!(
            Path::new("feature/test-feature/impl"),
            app_ctx.feature_impl_path("test-feature").as_ref()
        )
    }

    #[test]
    fn feature_impl_path_package_path() {
        let app_ctx = AppCtx::default();

        assert_eq!(
            Path::new("feature/test-feature/impl/src/main/kotlin/test/base/package"),
            app_ctx.feature_impl_package_path("test-feature").as_ref()
        )
    }

    #[test]
    fn feature_api_path() {
        let app_ctx = AppCtx::default();

        assert_eq!(
            Path::new("feature/test-feature/api"),
            app_ctx.feature_api_path("test-feature").as_ref()
        )
    }

    #[test]
    fn feature_api_path_package_path() {
        let app_ctx = AppCtx::default();

        assert_eq!(
            Path::new("feature/test-feature/api/src/main/kotlin/test/base/package"),
            app_ctx.feature_api_package_path("test-feature").as_ref()
        )
    }

    #[test]
    fn base_package_path_part() {
        let app_ctx = AppCtx::default();

        assert_eq!(
            Path::new("test/base/package"),
            app_ctx.base_package_path_part().as_ref()
        )
    }

    #[test]
    fn feature_name() {
        let feature = Feature {
            app_context: AppCtx::default(),
            feature_name: "test-feature".into(),
        };

        let root = FeatureGenCtx::Root(feature);

        assert_eq!("test-feature", root.feature_name())
    }

    impl Default for AppCtx {
        fn default() -> Self {
            Self {
                is_testing: false,
                app_name: "test_app_name".into(),
                base_package: "test.base.package".into(),
            }
        }
    }

    impl Cli {
        fn with_command(command: Command) -> Self {
            Cli {
                debug: false,
                base_package: Some("test.base.package".into()),
                app_name: Some("test_app_name".into()),
                command,
            }
        }
    }
}
