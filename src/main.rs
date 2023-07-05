use std::collections::BTreeMap;
use std::fs::{create_dir_all, File, OpenOptions, read_to_string};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use convert_case::{Case, Casing};
use serde::Serialize;
use handlebars::{Handlebars};
use crate::args_parser::{Cli, Commands, get_global_config_path, get_local_config_path};

mod helpers;
mod args_parser;

// impl/di
const FEATURE_ENTRY_MODULE: &str = include_str!("templates/impl/di/FeatureEntryModule.handlebars");
const FEATURE_ROOT_COMPONENT: &str = include_str!("templates/impl/di/FeatureRootComponent.handlebars");
const ROOT_MODULE: &str = include_str!("templates/impl/di/RootModule.handlebars");
const SUBCOMPONENTS_MODULE: &str = include_str!("templates/impl/di/SubcomponentsModule.handlebars");

// impl/directions
const FEATURE_DIRECTIONS: &str = include_str!("templates/impl/directions/FeatureDirections.handlebars");
const SUBFEATURE_DIRECTION_TEMPLATE: &str = include_str!("templates/impl/directions/SubfeatureDirectionTemplate.handlebars");

// impl/entry
const FEATURE_ENTRY_IMPL: &str = include_str!("templates/impl/entry/FeatureEntryImpl.handlebars");
const SUBFEATURE_COMPOSABLE_TEMPLATE: &str = include_str!("templates/impl/entry/SubfeatureComposableTemplate.handlebars");

// impl/firstpage
const PAGE_MODULE: &str = include_str!("templates/impl/firstpage/di/FirstPageModule.handlebars");
const PAGE_SUBCOMPONENT: &str = include_str!("templates/impl/firstpage/di/FirstPageSubcomponent.handlebars");
const PAGE_SCREEN: &str = include_str!("templates/impl/firstpage/screen/FirstPageScreen.handlebars");
const PAGE_VIEW_MODEL: &str = include_str!("templates/impl/firstpage/screen/FirstPageViewModel.handlebars");
const PAGE_USE_CASE: &str = include_str!("templates/impl/firstpage/usecase/FirstPageUseCase.handlebars");

// impl/build
const IMPL_BUILD: &str = include_str!("templates/impl/ImplBuild.handlebars");

// api
const API_BUILD: &str = include_str!("templates/api/ApiBuild.handlebars");
const API_FEATURE_ENTRY: &str = include_str!("templates/api/FeatureEntry.handlebars");
const API_FEATURE_PROVIDER: &str = include_str!("templates/api/ApiFeatureProvider.handlebars");

// etc
const SETTINGS_GRADLE: &str = include_str!("templates/etc/settings.gradle.kts.handlebars");
const DEPENDENCY_MODULE_EXTENSION: &str = include_str!("templates/etc/dependency-module-extension.handlebars");

// mock
const MOCK_NAVIGATION_ENTRY_MODULE: &str = include_str!("templates/mock/MockNavigationEntryModule.kt.handlebars");
const MOCK_BUILD_SRC_MODULES: &str = include_str!("templates/mock/MockBuildSrcModules.kt.handlebars");
const MOCK_APP_BUILD_GRADLE: &str = include_str!("templates/mock/MockAppBUildGradle.handlebars");

fn main() {
    let mut handlebars = Handlebars::new();
    register_helpers(&mut handlebars);

    let args = args_parser::parse_args();
    match args.command {
        Commands::GenMod { .. } => {
            gen_mod(&mut handlebars, &args);
        }
        Commands::GenScreen { .. } => {
            gen_screen(&mut handlebars, &args);
        }
        Commands::Config { global, ref app_name, ref base_package } => {
            gen_config(&args, global, app_name, base_package);
        }
    }
}

fn gen_config(args: &Cli, global: bool, app_name: &Option<String>, base_package: &Option<String>) {
    let config_path = if global {
        get_global_config_path()
    } else {
        get_local_config_path()
    };
    create_dir_all(config_path.parent().unwrap()).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(config_path)
        .unwrap();

    let mut lines = Vec::new();
    if let Some(package) = base_package.as_ref().or(args.base_package.as_ref()) {
        lines.push(format!("base-package = \"{}\"", package));
    }
    if let Some(name) = app_name.as_ref().or(args.app_name.as_ref()) {
        lines.push(format!("app-name = \"{}\"", name));
    }

    file.write_all(lines.join("\n").as_bytes()).unwrap();
}

fn gen_screen(handlebars: &mut Handlebars, args: &Cli) {
    let context = build_context(&args);
    let module = context.get("module").unwrap();
    let page: &String = context.get("first_page").unwrap();
    let base_package = args.base_package.as_ref().unwrap().split(".").collect::<Vec<&str>>().join("/");
    let root = Path::new(if args.debug { "./test/" } else { "./" });
    let feature = format!("feature/{}", module.to_case(Case::Kebab));
    let feature_root = root.join(&feature);
    let impl_root = feature_root.join("impl");

    let base_impl_package_path = impl_root
        .join("src/main/java")
        .join(&base_package)
        .join(module.to_case(Case::Flat))
        .join("impl");

    generate_page(&base_impl_package_path, handlebars, &context, page);
    add_subcomponent_to_component(page, module, &base_impl_package_path, args.base_package.as_ref().unwrap());
    amend_directions(handlebars, &context, module, &base_impl_package_path);
    amend_feature_entry(handlebars, &context, module, &base_impl_package_path);
}

fn amend_feature_entry(handlebars: &mut Handlebars, context: &BTreeMap<String, String>, module: &String, base_impl_package_path: &PathBuf) {
    let feature_entry_path = base_impl_package_path.join(format!("entry/{}FeatureEntryImpl.kt", module.to_case(Case::Pascal)));
    let mut lines = read_to_string(&feature_entry_path).unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    add_line_after_matching_predicate(
        &mut lines,
        &|l| l.starts_with("    override fun FeatureGraphBuilderScope.buildNavigation()"),
        handlebars.render_template(SUBFEATURE_COMPOSABLE_TEMPLATE, &context).unwrap().as_str(),
    );

    add_line_after_matching_predicate(
        &mut lines,
        &|l| l.ends_with("Screen") && l.starts_with("import"),
        handlebars.render_template(
            "import {{ base_package }}.{{ flat module }}.impl.{{ flat first_page }}.screen.{{ pascal first_page }}Screen\nimport {{ base_package }}.{{ flat module }}.impl.{{ flat first_page }}.di.{{ pascal first_page }}Subcomponent",
            &context).unwrap().as_str(),
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&feature_entry_path)
        .unwrap();

    file.write(lines.join("\n").as_bytes()).unwrap();
}

fn amend_directions(handlebars: &mut Handlebars, context: &BTreeMap<String, String>, module: &String, base_impl_package_path: &PathBuf) {
    let directions_path = base_impl_package_path.join("directions").join(format!("{}Directions.kt", module.to_case(Case::Pascal)));
    let mut lines = read_to_string(&directions_path).unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();


    if lines.iter().find(|l| l.ends_with("EmptyInput")).is_none() {
        add_line_after_matching_predicate(
            &mut lines,
            &|l| l.ends_with("NavigationCommandProvider"),
            format!("import {}.navigation.EmptyInput", context.get("base_package").unwrap()).as_str(),
        );
    }

    if lines.iter().find(|l| l.eq_ignore_ascii_case("import androidx.navigation.NamedNavArgument")).is_none() {
        add_line_after_matching_predicate(
            &mut lines,
            &|l| l.contains("androidx.navigation."),
            "import androidx.navigation.NamedNavArgument",
        );
    }

    add_line_after_matching_predicate(
        &mut lines,
        &|l| l.starts_with("object"),
        handlebars.render_template(SUBFEATURE_DIRECTION_TEMPLATE, &context).unwrap().as_str(),
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&directions_path)
        .unwrap();

    file.write(lines.join("\n").as_bytes()).unwrap();
}

fn add_subcomponent_to_component(screen_name: &str, module: &str, base_impl_package_path: &Path, base_package: &str) {
    // ===== component =====
    let component_path = base_impl_package_path.join("di").join(format!("{}RootComponent.kt", module.to_case(Case::Pascal)));
    let mut lines = read_to_string(&component_path).unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let base_impl_package = format!("{}.{}.impl", base_package, module.to_case(Case::Flat));

    add_line_after_matching_predicate(
        &mut lines,
        &|l| l.ends_with("Subcomponent"),
        &format!("import {}.{}.di.{}Subcomponent", base_impl_package, screen_name.to_case(Case::Flat), screen_name.to_case(Case::Pascal)),
    );

    add_line_after_matching_predicate(
        &mut lines,
        &|l| l.ends_with("Subcomponent.Factory"),
        &format!("    val {}SubcomponentFactory: {}Subcomponent.Factory", screen_name.to_case(Case::Camel), screen_name.to_case(Case::Pascal)),
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&component_path)
        .unwrap();

    file.write(lines.join("\n").as_bytes()).unwrap();

    // ===== SubcomponentsModule ======
    let subcomponents_module_path = base_impl_package_path.join("di").join(format!("{}SubcomponentsModule.kt", module.to_case(Case::Pascal)));
    let mut lines = read_to_string(&subcomponents_module_path).unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    add_line_after_matching_predicate(
        &mut lines,
        &|l| l.ends_with("Subcomponent"),
        &format!("import {}.{}.di.{}Subcomponent", base_impl_package, screen_name.to_case(Case::Flat), screen_name.to_case(Case::Pascal)),
    );

    add_line_after_matching_predicate(
        &mut lines,
        &|l| l.ends_with("Subcomponent::class,"),
        &format!("        {}Subcomponent::class,", screen_name.to_case(Case::Pascal)),
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&subcomponents_module_path)
        .unwrap();

    file.write(lines.join("\n").as_bytes()).unwrap();
}

fn add_line_after_matching_predicate(
    content: &mut Vec<String>,
    predicate: &dyn Fn(&str) -> bool,
    line: &str,
) {
    let mut index = 0;
    for (i, l) in content.iter().enumerate() {
        if predicate(l) {
            index = i;
            break;
        }
    }
    content.insert(index + 1, line.to_string());
}

fn gen_mod(mut handlebars: &mut Handlebars, args: &Cli) {
    let context = build_context(&args);
    let module = context.get("module").unwrap();
    let dotted_base_package = context.get("base_package").unwrap().clone();
    let first_page: &String = context.get("first_page").unwrap();
    let app_name = context.get("app").unwrap();
    let is_test = args.debug;
    let base_package = dotted_base_package.split(".").collect::<Vec<&str>>().join("/");
    let root = Path::new(if is_test { "./test/" } else { "./" });
    let feature = format!("feature/{}", module.to_case(Case::Kebab));
    let feature_root = root.join(&feature);
    let api_root = feature_root.join("api");
    let impl_root = feature_root.join("impl");

    let base_api_package = api_root
        .join("src/main/java")
        .join(&base_package)
        .join(module.to_case(Case::Flat))
        .join("api");

    let base_impl_package = impl_root
        .join("src/main/java")
        .join(&base_package)
        .join(module.to_case(Case::Flat))
        .join("impl");

    create_dir_all(&base_api_package).unwrap();
    create_dir_all(&base_impl_package).unwrap();

    generate_api_files(&api_root, &base_api_package, &handlebars, &context, &module);
    generate_impl_files(&impl_root, &base_impl_package, &handlebars, &context, module, first_page);

    add_feature_to_settings(&mut handlebars, &context, is_test, root);
    amend_navigation_entry_module(&mut handlebars, &context, module, dotted_base_package, app_name, is_test, base_package, root);
    amend_build_src(&mut handlebars, &context, is_test, root);
    amend_app_build_gradle(&mut handlebars, &context, module, is_test, root);
}

fn amend_navigation_entry_module(handlebars: &mut Handlebars, context: &BTreeMap<String, String>, module: &String, dotted_base_package: String, app_name: &String, is_test: bool, base_package: String, root: &Path) {
    let path = format!("app/src/main/java/{}/{}", base_package, app_name);
    let app_package = Path::new(&path);
    let app_package = root.join(app_package);
    let di_package = app_package.join("di");

    let navigation_entry_module = di_package.join("NavigationEntryModule.kt");
    if is_test {
        create_dir_all(navigation_entry_module.parent().unwrap()).unwrap();
        let mock_file = File::create(&navigation_entry_module).unwrap();
        handlebars.render_template_to_write(MOCK_NAVIGATION_ENTRY_MODULE, &context, &mock_file).unwrap();
    }

    let navigation_entry_module_content = read_to_string(&navigation_entry_module).unwrap();
    let lines: Vec<String> = add_import_and_include_to_navigation_entry_module(module, dotted_base_package, navigation_entry_module_content);

    let mut navigation_entry_module = OpenOptions::new()
        .create(is_test)
        .write(true)
        .truncate(true)
        .open(navigation_entry_module)
        .unwrap();

    navigation_entry_module.write(lines.join("\n").as_bytes()).unwrap();
}

fn amend_app_build_gradle(handlebars: &mut Handlebars, context: &BTreeMap<String, String>, module: &String, is_test: bool, root: &Path) {
    let app_build_gradle = root.join("app/build.gradle.kts");
    if is_test {
        create_dir_all(app_build_gradle.parent().unwrap()).unwrap();
        let mock_file = File::create(&app_build_gradle).unwrap();
        handlebars.render_template_to_write(MOCK_APP_BUILD_GRADLE, &context, &mock_file).unwrap();
    }

    let app_build_gradle_content = read_to_string(&app_build_gradle).unwrap();
    let lines: Vec<String> = app_build_gradle_content.lines()
        .fold(Vec::new(), |mut acc, line| {
            let line = line.to_string();
            acc.push(line.clone());
            if line.contains("// ===== feature modules =====") {
                let dependency_implementation = format!("    implementation(*{}.all())", module.to_case(Case::Camel));
                acc.push(dependency_implementation)
            }
            acc
        });

    let mut app_build_gradle = OpenOptions::new()
        .create(is_test)
        .write(true)
        .truncate(true)
        .open(app_build_gradle)
        .unwrap();

    app_build_gradle.write(lines.join("\n").as_bytes()).unwrap();
}

fn amend_build_src(handlebars: &mut Handlebars, context: &BTreeMap<String, String>, is_test: bool, root: &Path) {
    let path = format!("buildSrc/src/main/kotlin/");
    let build_src_root = Path::new(&path);
    let build_src_root = root.join(build_src_root);

    let build_src_modules = build_src_root.join("app-modules.kt");
    if is_test {
        create_dir_all(build_src_modules.parent().unwrap()).unwrap();
        let mock_file = File::create(&build_src_modules).unwrap();
        handlebars.render_template_to_write(MOCK_BUILD_SRC_MODULES, &context, &mock_file).unwrap();
    }

    let build_src_modules_content = read_to_string(&build_src_modules).unwrap();
    let lines: Vec<String> = build_src_modules_content.lines()
        .fold(Vec::new(), |mut acc, line| {
            let line = line.to_string();
            acc.push(line.clone());
            if line.contains("// ===== feature modules =====") {
                let dependency_extension = handlebars.render_template(DEPENDENCY_MODULE_EXTENSION, &context).unwrap();
                acc.push(dependency_extension)
            }
            acc
        });

    let mut build_src_modules = OpenOptions::new()
        .create(is_test)
        .write(true)
        .truncate(true)
        .open(build_src_modules)
        .unwrap();

    build_src_modules.write(lines.join("\n").as_bytes()).unwrap();
}

fn add_import_and_include_to_navigation_entry_module(module: &String, dotted_base_package: String, navigation_entry_module_content: String) -> Vec<String> {
    navigation_entry_module_content.lines()
        .fold(Vec::new(), |mut acc, line| {
            let line = line.to_string();
            if line.contains("dagger.Module") {
                let import_statement = format!(
                    "import {0}.{1}.impl.di.{2}FeatureEntryModule",
                    dotted_base_package,
                    module.to_case(Case::Flat),
                    module.to_case(Case::Pascal)
                );

                acc.push(import_statement);
                acc.push(line);
            } else if line.contains("    includes = [") {
                acc.push(line);
                let module_class = format!("        {0}FeatureEntryModule::class,", module.to_case(Case::Pascal));
                acc.push(module_class);
            } else {
                acc.push(line);
            }
            acc
        })
}

fn add_feature_to_settings<T: Serialize>(handlebars: &mut Handlebars, context: &T, is_test: bool, root: &Path) {
    let settings_import = handlebars.render_template(SETTINGS_GRADLE, &context).unwrap();

    let mut settings_gradle = OpenOptions::new()
        .create(is_test)
        .write(true)
        .append(true)
        .open(root.join("settings.gradle.kts"))
        .unwrap();

    if let Err(e) = writeln!(settings_gradle, "{}", settings_import) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn generate_api_files<T: Serialize>(
    api_root: &Path,
    base_api_package: &Path,
    handlebars: &Handlebars,
    data: &T,
    module: &str,
) {
    generate_file(api_root, handlebars, data, "build.gradle.kts", API_BUILD);
    let file_name = format!("{}FeatureEntry.kt", module.to_case(Case::Pascal));
    generate_file(base_api_package, handlebars, data, &file_name, API_FEATURE_ENTRY);
    let file_name = format!("{}Provider.kt", module.to_case(Case::Pascal));
    generate_file(base_api_package, handlebars, data, &file_name, API_FEATURE_PROVIDER)
}

fn generate_impl_files<T: Serialize>(
    impl_root: &Path,
    base_impl_package: &Path,
    handlebars: &Handlebars,
    data: &T,
    module: &str,
    first_page: &str,
) {
    generate_file(impl_root, handlebars, data, "build.gradle.kts", IMPL_BUILD);

    generate_impl_di(base_impl_package, handlebars, data, module);
    generate_impl_entry(base_impl_package, handlebars, data, module);
    generate_directions(base_impl_package, handlebars, data, module);
    generate_page(base_impl_package, handlebars, data, first_page);
}

fn generate_directions<T: Serialize>(base_impl_package: &Path, handlebars: &Handlebars, data: &T, module: &str) {
    let directions_package = base_impl_package.join("directions");
    let directions_package = directions_package.as_path();
    create_dir_all(directions_package).unwrap();

    let file_name = format!("{}Directions.kt", module.to_case(Case::Pascal));
    generate_file(&directions_package, handlebars, data, &file_name, FEATURE_DIRECTIONS);
}

fn generate_impl_di<T: Serialize>(base_impl_package: &Path, handlebars: &Handlebars, data: &T, module: &str) {
    let di_package = base_impl_package.join("di");
    let di_package = di_package.as_path();
    create_dir_all(di_package).unwrap();

    let file_name = format!("{}FeatureEntryModule.kt", module.to_case(Case::Pascal));
    generate_file(&di_package, handlebars, data, &file_name, FEATURE_ENTRY_MODULE);
    let file_name = format!("{}RootComponent.kt", module.to_case(Case::Pascal));
    generate_file(&di_package, handlebars, data, &file_name, FEATURE_ROOT_COMPONENT);
    let file_name = format!("{}RootModule.kt", module.to_case(Case::Pascal));
    generate_file(&di_package, handlebars, data, &file_name, ROOT_MODULE);
    let file_name = format!("{}SubcomponentsModule.kt", module.to_case(Case::Pascal));
    generate_file(&di_package, handlebars, data, &file_name, SUBCOMPONENTS_MODULE);
}

fn generate_impl_entry<T: Serialize>(base_impl_package: &Path, handlebars: &Handlebars, data: &T, module: &str) {
    let entry_package = base_impl_package.join("entry");
    let entry_package = entry_package.as_path();
    create_dir_all(entry_package).unwrap();

    let file_name = format!("{}FeatureEntryImpl.kt", module.to_case(Case::Pascal));
    generate_file(&entry_package, handlebars, data, &file_name, FEATURE_ENTRY_IMPL);
}

fn generate_page<T: Serialize>(base_impl_package: &Path, handlebars: &Handlebars, data: &T, first_page: &str) {
    let first_page_package = first_page.to_string();
    let first_page_package = base_impl_package.join(first_page_package.to_case(Case::Flat));
    let first_page_package = first_page_package.as_path();
    create_dir_all(first_page_package).unwrap();

    // ===== di package =====
    let di_package = first_page_package.join("di");
    let di_package = di_package.as_path();
    create_dir_all(di_package).unwrap();

    let file_name = format!("{}Module.kt", first_page.to_case(Case::Pascal));
    generate_file(&di_package, handlebars, data, &file_name, PAGE_MODULE);
    let file_name = format!("{}Subcomponent.kt", first_page.to_case(Case::Pascal));
    generate_file(&di_package, handlebars, data, &file_name, PAGE_SUBCOMPONENT);

    // ===== screen package =====
    let screen_package = first_page_package.join("screen");
    let screen_package = screen_package.as_path();
    create_dir_all(screen_package).unwrap();

    let file_name = format!("{}Screen.kt", first_page.to_case(Case::Pascal));
    generate_file(&screen_package, handlebars, data, &file_name, PAGE_SCREEN);
    let file_name = format!("{}ScreenViewModel.kt", first_page.to_case(Case::Pascal));
    generate_file(&screen_package, handlebars, data, &file_name, PAGE_VIEW_MODEL);

    // ===== usecase package =====
    let usecase_package = first_page_package.join("usecase");
    let usecase_package = usecase_package.as_path();
    create_dir_all(usecase_package).unwrap();

    let file_name = format!("{}UseCase.kt", first_page.to_case(Case::Pascal));
    generate_file(&usecase_package, handlebars, data, &file_name, PAGE_USE_CASE);
}

fn generate_file<T: Serialize>(parent: &Path, handlebars: &Handlebars, data: &T, file_name: &str, template_content: &str) {
    let file = File::create(parent.join(file_name)).unwrap();
    handlebars.render_template_to_write(template_content, data, file).unwrap();
}

fn build_context(args: &Cli) -> BTreeMap<String, String> {
    let mut data = BTreeMap::new();
    data.insert("base_package".to_string(), args.base_package.as_ref().unwrap().clone());
    data.insert("app".to_string(), args.app_name.as_ref().unwrap().clone());
    match &args.command {
        Commands::GenMod { feature, start_screen } => {
            data.insert("module".to_string(), feature.clone());
            data.insert("first_page".to_string(), start_screen.clone());
        }
        Commands::GenScreen { feature, screen } => {
            data.insert("module".to_string(), feature.clone());
            data.insert("first_page".to_string(), screen.clone());
        }
        _ => { panic!("Trying to build context for non generate command") }
    }
    data
}

fn register_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("flat", Box::new(helpers::to_flat));
    handlebars.register_helper("pascal", Box::new(helpers::to_pascal));
    handlebars.register_helper("camel", Box::new(helpers::to_camel));
    handlebars.register_helper("kebab", Box::new(helpers::to_kebab));
}
