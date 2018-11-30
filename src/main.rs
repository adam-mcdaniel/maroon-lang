// use std::thread;
use std::env;
use std::process;

extern crate tools;

use tools::evaluator::*;
use tools::io_tools::*;
use tools::logging::*;
use tools::preprocessor::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        error("No filename");
        process::exit(0);
    }

    let mut preprocessor = Preprocessor::new();

    for line in Preprocessor::get_expressions(readlines(&args[1])) {
        Evaluator::new(&preprocessor.process(&line), &line).eval();
    }
    // TODO: Add import function / keyword
}
