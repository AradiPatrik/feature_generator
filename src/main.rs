use std::collections::BTreeMap;
use std::env;
use std::fmt::{Debug, format};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use convert_case::{Case, Casing};
use serde::Serialize;
use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};
use crate::helpers::to_kebab;

mod helpers;

// impl/di
const FEATURE_ENTRY_MODULE: &str = include_str!("templates/impl/di/FeatureEntryModule.handlebars");
const FEATURE_ROOT_COMPONENT: &str = include_str!("templates/impl/di/FeatureRootComponent.handlebars");
const ROOT_MODULE: &str = include_str!("templates/impl/di/RootModule.handlebars");
const SUBCOMPONENTS_MODULE: &str = include_str!("templates/impl/di/SubcomponentsModule.handlebars");

// impl/entry
const FEATURE_ENTRY_IMPL: &str = include_str!("templates/impl/entry/FeatureEntryImpl.handlebars");

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
const SETTINGS_GRADLE: &str = include_str!("templates/etc/settings.gradle.handlebars");

fn main() {
    let mut handlebars = Handlebars::new();
    register_helpers(&mut handlebars);

    let context = parse_parameters();
    let module = context.get("module").unwrap();
    let base_package = context.get("base_package").unwrap().clone();
    let first_page: &String = context.get("first_page").unwrap();
    let is_test = context.get("test_option").unwrap().split("=").nth(1).unwrap() == "true";
    let base_package = base_package.split(".").collect::<Vec<&str>>().join("/");
    let root = Path::new(if is_test { "./test/" } else { "./" });
    let feature = format!("feature/{}", module.to_case(Case::Kebab));
    let root = root.join(&feature);
    let api_root = root.join("api");
    let impl_root = root.join("impl");

    let base_api_package = api_root
        .join("src/java")
        .join(&base_package)
        .join("api")
        .join(module.to_case(Case::Flat));

    let base_impl_package = impl_root
        .join("src/java")
        .join(&base_package)
        .join("impl")
        .join(module.to_case(Case::Flat));

    create_dir_all(&base_api_package).unwrap();
    create_dir_all(&base_impl_package).unwrap();

    generate_api_files(&api_root, &base_api_package, &handlebars, &context, &module);
    generate_impl_files(&impl_root, &base_impl_package, &handlebars, &context, module, first_page);

    let settings_import = handlebars.render_template(SETTINGS_GRADLE, &context).unwrap();

    let mut settings_gradle = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./settings.gradle.kts.handlebars")
        .unwrap();


    if let Err(e) = writeln!(settings_gradle, settings_import) {
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
    generate_first_page(base_impl_package, handlebars, data, first_page);
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

fn generate_first_page<T: Serialize>(base_impl_package: &Path, handlebars: &Handlebars, data: &T, first_page: &str) {
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

fn parse_parameters() -> BTreeMap<String, String> {
    let mut data = BTreeMap::new();

    let args: Vec<String> = env::args().collect();

    let base_package = &args[1];
    let first_page = &args[3];
    let module_name = &args[2];
    let app_name = &args[4];
    let test_option = if args.len() == 6 {
        Some(&args[5])
    } else {
        None
    };

    data.insert("module".to_string(), module_name.to_string());
    data.insert("base_package".to_string(), base_package.to_string());
    data.insert("first_page".to_string(), first_page.to_string());
    data.insert("app".to_string(), app_name.to_string());
    data.insert("test_option".to_string(), test_option.unwrap_or(&"test=false".to_string()).to_string());
    data
}

fn register_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("flat", Box::new(helpers::to_flat));
    handlebars.register_helper("pascal", Box::new(helpers::to_pascal));
    handlebars.register_helper("camel", Box::new(helpers::to_camel));
    handlebars.register_helper("kebab", Box::new(helpers::to_kebab));
}
