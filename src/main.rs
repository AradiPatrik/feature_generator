mod helpers;

use std::collections::BTreeMap;
use std::env;
use convert_case::{Case, Casing};
use handlebars::{Handlebars, RenderContext, Helper, Context, JsonRender, HelperResult, Output};

fn main() {
    let args: Vec<String> = env::args().collect();

    let base_package = &args[1];
    let first_page = &args[2];
    let name = &args[3];

    let mut handlebars = Handlebars::new();
    register_helpers(&mut handlebars);

    let source = include_str!("FeatureComponent.handlebars");
    assert!(handlebars.register_template_string("feature_component", source).is_ok());

    let mut data = BTreeMap::new();
    data.insert("name".to_string(), name);
    data.insert("base_package".to_string(), base_package);
    data.insert("first_page_name".to_string(), first_page);

    println!("{}", handlebars.render("feature_component", &data).unwrap())
}

fn register_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("to_flat", Box::new(helpers::to_flat));
    handlebars.register_helper("to_pascal", Box::new(helpers::to_pascal));
}
