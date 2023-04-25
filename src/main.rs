mod helpers;

use std::collections::BTreeMap;
use std::env;
use convert_case::{Case, Casing};
use handlebars::{Handlebars, RenderContext, Helper, Context, JsonRender, HelperResult, Output};

fn main() {
    let args: Vec<String> = env::args().collect();

    let base_package = &args[1];
    let first_page = &args[3];
    let module_name = &args[2];
    let app_name = &args[4];

    let mut handlebars = Handlebars::new();
    register_helpers(&mut handlebars);

    let source = include_str!("templates/api/FeatureEntry.handlebars");
    assert!(handlebars.register_template_string("feature_component", source).is_ok());

    let mut data = BTreeMap::new();
    data.insert("module".to_string(), module_name);
    data.insert("base_package".to_string(), base_package);
    data.insert("first_page".to_string(), first_page);
    data.insert("app".to_string(), app_name);

    println!("{}", handlebars.render("feature_component", &data).unwrap())
}

fn register_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("flat", Box::new(helpers::to_flat));
    handlebars.register_helper("pascal", Box::new(helpers::to_pascal));
    handlebars.register_helper("camel", Box::new(helpers::to_camel));
    handlebars.register_helper("kebab", Box::new(helpers::to_kebab));
}
