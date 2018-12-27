use std::env;
use std::process;

extern crate tools;

use tools::evaluator::*;
use tools::io_tools::*;
use tools::logging::*;
// use tools::string_tools::*;
use tools::preprocessor::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        error("No filename");
        process::exit(0);
    }

    let mut preprocessor = Preprocessor::new();

    let script = readlines(&args[1]);
    let processed_script = Preprocessor::get_expressions(script);

    // println!("{:?}", processed_script);

    for line in Preprocessor::get_expressions(processed_script) {
        Evaluator::new(&preprocessor.process(&line), &line).eval();
    }
}
