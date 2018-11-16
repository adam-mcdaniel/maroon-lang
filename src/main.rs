// use std::thread;
use std::env;
use std::process;

mod tools;

use tools::logging::*;
use tools::io_tools::*;
use tools::evaluator::*;
use tools::preprocessor::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        error("No filename");
        process::exit(0);
    }

    let mut preprocessor = Preprocessor::new();

    for line in Preprocessor::get_expressions(readlines(&args[1])) {
        Evaluator::new(
            &preprocessor.process(
                &line
            ), &line).eval();
    }







    // succ := n.(f.(x.(f[n[f][x]])))


    // println!("{}",
    //     to_primitive_call(
    //         "a.(b.b) [0][1]"
    //     )
    // );


    // concurrent
    // info(
    //     thread::spawn(|| {
    //         Evaluator::new(
    //             "a.(a + 1) (fun.(function fun !)) !"
    //         ).eval().join(" ")
    //     }).join().unwrap()
    // );
    // info(
    //     thread::spawn(|| {
    //         Evaluator::new(
    //             "a.(a !) (x.( 8 c.c x ! )) !"
    //         ).eval().join(" ")
    //     }).join().unwrap()
    // );
    // info(
    //     thread::spawn(|| {
    //         Evaluator::new(
    //             "28 (a.(a + 1)) !"
    //         ).eval().join(" ")
    //     }).join().unwrap()
    // );

    // procedural

    // T = (a.b.a)
    // F = (a.b.b)
    // and = (p.q.(  (p q !) p ! ))

    // debug(&to_primitive_call(
    //     "(a.(b.a)) [0][1]
        
    //     @STDOUT \\n @STDOUT
    //     "));


    // println!("len: {}", "(a.(a a !) b.(b b !) !)".len());
    // Evaluator::new(
    //     &to_primitive_call(
    //         "(a.(a a !) b.(b b !) !)"
    //     )).eval().join("");


    // info(Evaluator::new("a.(a tion) (fun.(func fun !)) !").eval().join(""));
    // info(Evaluator::new("a.(a !) (x.( 8 c.c x ! )) !").eval().join(""));
    // info(Evaluator::new("28 (a.(a 1)) !").eval().join(""));

}