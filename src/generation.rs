use std::path::Path;

use crate::args_parser;
use crate::{args_parser::Cli, helpers};
use convert_case::Case;
use convert_case::Casing;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use self::gen_context::AppCtx;
use self::gen_context::AppGenCtx;
use self::gen_context::Config;
use self::gen_context::CtxCreationError;
use self::gen_context::FeatureGenCtx;
use self::gen_context::GenCtx;
use self::gen_context::Library;
use self::render::append_line_below;
use self::render::{add_line_to_file, overwrite_file_at_path};

pub mod gen_context;
pub mod render;

pub use gen_context::Feature;
pub use gen_context::Subfeature;

const API_FEATURE_ENTRY: &str = include_str!("templates/api/FeatureEntry.handlebars");
const API_BUILD: &str = include_str!("templates/api/ApiBuild.handlebars");
const IMPL_BUILD: &str = include_str!("templates/impl/ImplBuild.handlebars");
const FEATURE_ROOT: &str = include_str!("templates/impl/root/FeatureRoot.handlebars");
const SUBFEATURE: &str = include_str!("./templates/impl/firstpage/Subfeature.handlebars");
const PAGE_SCREEN: &str =
    include_str!("templates/impl/firstpage/screen/FirstPageScreen.handlebars");
const PAGE_VIEW_MODEL: &str =
    include_str!("templates/impl/firstpage/screen/FirstPageViewModel.handlebars");

pub fn register_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("flat", Box::new(helpers::to_flat));
    handlebars.register_helper("pascal", Box::new(helpers::to_pascal));
    handlebars.register_helper("camel", Box::new(helpers::to_camel));
    handlebars.register_helper("kebab", Box::new(helpers::to_kebab));
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HandlebarsContext {
    base_package: Option<String>,
    app: Option<String>,
    module: Option<String>,
    first_page: Option<String>,
}

impl HandlebarsContext {
    pub fn new(generation_context: &GenCtx) -> Self {
        let base_package = generation_context.app_ctx().map(|ctx| &ctx.base_package);
        let app = generation_context.app_ctx().map(|ctx| &ctx.app_name);
        let module = if let GenCtx::App(AppGenCtx::Feature(feature)) = generation_context {
            Some(feature.feature_name().to_string())
        } else {
            None
        };
        let first_page =
            if let GenCtx::App(AppGenCtx::Feature(FeatureGenCtx::Subfeature(subfeature))) =
                generation_context
            {
                Some(subfeature.subfeature_name.to_string())
            } else if let GenCtx::App(AppGenCtx::Feature(FeatureGenCtx::Root(root))) =
                generation_context
            {
                Some(root.feature_name.to_string())
            } else {
                None
            };

        Self {
            base_package: base_package.cloned(),
            app: app.cloned(),
            module,
            first_page,
        }
    }
}

#[derive(Debug)]
pub struct Generator<'a> {
    handlebars: Handlebars<'a>,
    handlebars_context: HandlebarsContext,
    generation_context: GenCtx,
}

impl<'a> Generator<'a> {
    fn from_cli_internal(
        cli: Cli,
        mut register_helpers: impl FnMut(&mut Handlebars),
    ) -> Result<Self, CtxCreationError> {
        let mut handlebars = Handlebars::new();
        register_helpers(&mut handlebars);
        let generation_context = GenCtx::from_cli(cli.clone())?;
        let handlebars_context = HandlebarsContext::new(&generation_context);

        Ok(Generator {
            handlebars,
            handlebars_context,
            generation_context,
        })
    }

    pub fn new(generation_context: GenCtx) -> Self {
        let mut handlebars = Handlebars::new();
        register_helpers(&mut handlebars);

        Generator {
            handlebars,
            handlebars_context: HandlebarsContext::new(&generation_context),
            generation_context,
        }
    }

    pub fn from_cli(cli: Cli) -> Result<Self, CtxCreationError> {
        Self::from_cli_internal(cli, &register_helpers)
    }

    pub fn generate(&self) {
        match &self.generation_context {
            GenCtx::App(ref app_gen_ctx) => self.generate_app(app_gen_ctx),
            GenCtx::Config(ref config_ctx) => self.generate_config(config_ctx),
        }
    }

    fn generate_app(&self, app_gen_ctx: &'a AppGenCtx) {
        match app_gen_ctx {
            AppGenCtx::Feature(feature_gen_ctx) => self.generate_feature(feature_gen_ctx),
            AppGenCtx::Library(library) => self.generate_library(library),
        }
    }

    fn generate_feature(&self, feature: &FeatureGenCtx) {
        match feature {
            FeatureGenCtx::Root(root) => self.generate_feature_root(root),
            FeatureGenCtx::Subfeature(subfeature) => self.generate_subfeature(subfeature),
        }
    }

    fn generate_feature_root(&self, feature: &Feature) {
        self.generate_file(
            &feature
                .app_context
                .feature_api_package_path(&feature.feature_name),
            &format!(
                "{}FeatureEntry.kt",
                feature.feature_name.to_case(Case::Pascal)
            ),
            API_FEATURE_ENTRY,
        );

        self.generate_file(
            &feature.app_context.feature_api_path(&feature.feature_name),
            "build.gradle.kts",
            API_BUILD,
        );

        self.generate_file(
            &feature.app_context.feature_impl_path(&feature.feature_name),
            "build.gradle.kts",
            IMPL_BUILD,
        );

        self.generate_file(
            &self.feature_root_path(&feature.app_context, &feature.feature_name),
            &format!(
                "{}FeatureRoot.kt",
                &feature.feature_name.to_case(Case::Pascal)
            ),
            FEATURE_ROOT,
        );

        self.generate_subfeature(&Subfeature {
            app_context: feature.app_context.clone(),
            feature_name: feature.feature_name.clone(),
            subfeature_name: feature.feature_name.clone(),
        });

        let application_class = self
            .application_root(&feature.app_context)
            .join("Application.kt");

        append_line_below(
            &application_class,
            "features =",
            &format!(
                "        {}FeatureRoot::class,",
                feature.feature_name.to_case(Case::Pascal)
            ),
        );

        append_line_below(
            &application_class,
            "import ",
            &format!(
                "import {}.{}.impl.root.{}FeatureRoot",
                feature.app_context.base_package,
                feature.feature_name.to_case(Case::Flat),
                feature.feature_name.to_case(Case::Pascal),
            ),
        );

        add_line_to_file(
            Path::new("settings.gradle.kts"),
            &format!(
                "include(\":feature:{}:api\")\ninclude(\":feature:{}:impl\")",
                &feature.feature_name.to_case(Case::Kebab),
                &feature.feature_name.to_case(Case::Kebab)
            ),
        );

        add_line_to_file(
            &Self::build_src_path().join("app-modules.kt"),
            &format!(
                "val DependencyHandlerScope.{} get() = createProject(\":feature:{}\")",
                &feature.feature_name.to_case(Case::Camel),
                &feature.feature_name.to_case(Case::Kebab)
            ),
        );

        append_line_below(
            Path::new("app/build.gradle.kts"),
            "dependencies",
            &format!(
                "    implementation(*{}.all())",
                &feature.feature_name.to_case(Case::Camel)
            ),
        );
    }

    fn build_src_path() -> Box<Path> {
        Path::new("buildSrc/src/main/kotlin").into()
    }

    fn application_root(&self, app_ctx: &AppCtx) -> Box<Path> {
        return Path::new("app")
            .join("src/main/java")
            .join(app_ctx.base_package_path_part())
            .join(app_ctx.app_name.to_case(Case::Flat))
            .into();
    }

    fn feature_root_path(&self, app_ctx: &AppCtx, feature_name: &str) -> Box<Path> {
        app_ctx
            .feature_impl_package_path(feature_name)
            .join("root")
            .into()
    }

    fn subfeature_package_path(&self, app_ctx: &AppCtx, feature_name: &str) -> Box<Path> {
        app_ctx
            .feature_impl_package_path(feature_name)
            .join("subfeature")
            .into()
    }

    fn generate_file(&self, path: &Path, file_name: &str, file_content: &str) {
        render::generate_file(
            path,
            &self.handlebars,
            &self.handlebars_context,
            file_name,
            file_content,
        )
    }

    fn generate_subfeature(&self, subfeature: &Subfeature) {
        self.generate_file(
            &self
                .subfeature_package_path(&subfeature.app_context, &subfeature.feature_name)
                .join(subfeature.subfeature_name.to_case(Case::Flat)),
            &format!(
                "{}Subfeature.kt",
                &subfeature.subfeature_name.to_case(Case::Pascal)
            ),
            SUBFEATURE,
        );

        self.generate_file(
            &self
                .subfeature_package_path(&subfeature.app_context, &subfeature.feature_name)
                .join(subfeature.subfeature_name.to_case(Case::Flat))
                .join("screen"),
            &format!(
                "{}Screen.kt",
                &subfeature.subfeature_name.to_case(Case::Pascal)
            ),
            PAGE_SCREEN,
        );

        self.generate_file(
            &self
                .subfeature_package_path(&subfeature.app_context, &subfeature.feature_name)
                .join(subfeature.subfeature_name.to_case(Case::Flat))
                .join("screen"),
            &format!(
                "{}ScreenViewModel.kt",
                &subfeature.subfeature_name.to_case(Case::Pascal)
            ),
            PAGE_VIEW_MODEL,
        );
    }

    fn generate_library(&self, _library: &Library) {
        todo!()
    }

    fn generate_config(&self, config: &Config) {
        let config_path = if config.global {
            args_parser::get_global_config_path()
        } else {
            args_parser::get_local_config_path()
        };
        let mut lines = Vec::new();
        if let Some(package) = config.base_package_name.as_ref() {
            lines.push(format!("base-package = \"{}\"", package));
        }
        if let Some(name) = config.app_name.as_ref() {
            lines.push(format!("app-name = \"{}\"", name));
        }

        overwrite_file_at_path(&config_path, lines);
    }
}

#[cfg(test)]
mod test {
    use crate::args_parser::Cli;
    use crate::args_parser::Command;
    use crate::generation::gen_context::Config;

    use super::gen_context::GenCtx;
    use super::Generator;
    use super::HandlebarsContext;

    #[test]
    fn generate_feature_handlebars_context() {
        let cli = Cli {
            debug: false,
            base_package: Some("test.base.package".to_string()),
            app_name: Some("test-app-name".to_string()),
            command: Command::GenFeat {
                feature: "test-feature".to_string(),
            },
        };

        let generation_context = GenCtx::from_cli(cli.clone()).unwrap();

        let handlebars_context = HandlebarsContext::new(&generation_context);
        assert_eq!(
            HandlebarsContext {
                base_package: Some("test.base.package".to_string()),
                app: Some("test-app-name".to_string()),
                module: Some("test-feature".to_string()),
                first_page: Some("test-feature".to_string()),
            },
            handlebars_context
        )
    }

    #[test]
    fn generate_subfeature_handlebars_context() {
        let cli = Cli {
            debug: false,
            base_package: Some("test.base.package".to_string()),
            app_name: Some("test-app-name".to_string()),
            command: Command::GenSubfeat {
                feature: "test-feature".to_string(),
                screen: "test-subfeature".to_string(),
            },
        };

        let generation_context = GenCtx::from_cli(cli.clone()).unwrap();

        let handlebars_context = HandlebarsContext::new(&generation_context);
        assert_eq!(
            HandlebarsContext {
                base_package: Some("test.base.package".to_string()),
                app: Some("test-app-name".to_string()),
                module: Some("test-feature".to_string()),
                first_page: Some("test-subfeature".to_string())
            },
            handlebars_context
        )
    }
    #[test]
    fn generate_config_handlebars_context() {
        let handlebars_context = HandlebarsContext::new(&GenCtx::Config(Config {
            base_package_name: Some("test.base.package".to_string()),
            app_name: Some("test-app-name".to_string()),
            global: true,
        }));

        assert_eq!(
            HandlebarsContext {
                base_package: None,
                app: None,
                module: None,
                first_page: None
            },
            handlebars_context
        )
    }

    #[test]
    fn new_generator() {
        let cli = Cli {
            debug: false,
            base_package: Some("test.base.package".to_string()),
            app_name: Some("test-app-name".to_string()),
            command: Command::Config {
                global: true,
                base_package: Some("test.base.package".to_string()),
                app_name: Some("test-app-name".to_string()),
            },
        };
        let mut register_called = false;
        let generator =
            Generator::from_cli_internal(cli.clone(), |_| register_called = true).unwrap();
        let generator_context = GenCtx::from_cli(cli.clone()).unwrap();
        let handlebars_context = HandlebarsContext::new(&generator_context);

        assert_eq!(&generator_context, &generator.generation_context);
        assert_eq!(&handlebars_context, &generator.handlebars_context);
        assert!(register_called);
    }
}
