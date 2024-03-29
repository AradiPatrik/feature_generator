use std::process::exit;

use feature_generator::args_parser;
use feature_generator::generation::{self, Generator};

fn main() {
    let args = args_parser::parse_args();
    if args.is_none() {
        exit(0);
    }

    match Generator::from_cli(args.expect("Command should be something other than generate-completions at this point")) {
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
