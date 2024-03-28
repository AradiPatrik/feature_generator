use feature_generator::args_parser;
use feature_generator::generation::{self, Generator};

fn main() {
    let args = args_parser::parse_args();
    match Generator::from_cli(args) {
        Ok(generator) => generator.generate(),
        Err(err) => match err {
            generation::gen_context::CtxCreationError::AppNameMissing => print!(
                "App name is missing from config, please run `feature_generator config` first"
            ),
            generation::gen_context::CtxCreationError::BasePackageNameMissing => print!(
                "App base package is missing from config, please run `feature_generator config` first"
            ),
        },
    }
}
