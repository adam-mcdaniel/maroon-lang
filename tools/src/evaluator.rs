use io_tools::*;
use logging::*;
use preprocessor::*;
use std::process;
use string_tools::*;
use std::cmp;

pub struct Evaluator {
    preserved_program: String,
    data: Vec<String>,
    logging: bool,
}

impl Evaluator {
    pub fn new(program: &str, actual_line: &str) -> Self {
        Self {
            preserved_program: actual_line.to_string(),
            data: split(program),
            logging: false,
            // logging: true,
        }
    }

    pub fn next(&mut self) -> Option<String> {
        let result: Option<String>;

        if self.data.len() > 0 {
            result = Some(self.data[0].clone());
            self.data = self.data[1..].to_vec();
        } else {
            result = None;
        }

        return result;
    }

    pub fn pop(&mut self) -> Option<String> {
        let result: Option<String>;

        if self.data.len() > 0 {
            result = self.data.pop();
        } else {
            result = None;
        }

        return result;
    }


    pub fn push(&mut self, n: Vec<String>) {
        if n.len() > 0 {
            self.data.extend(n);
        }
    }

    pub fn push_front(&mut self, n: Vec<String>) {
        if n.len() > 0 {
            let mut val = n.clone();
            val.append(&mut self.data);
            self.data = val;
        }
    }

    pub fn end(&mut self) -> bool {
        let mut is_end = true;
        for s in &self.data {
            if [
                "!".to_string(),
                "@exit".to_string(),
                "@rec".to_string(),
                "@eval".to_string(),
                "@input".to_string(),
                "@print".to_string(),
                "@print*".to_string(),
                "@print_pipe".to_string(),
                "@print_pipe*".to_string(),
                "@println".to_string(),
                "@println*".to_string(),
            ]
                .contains(s)
            {
                is_end = false;
            }
        }

        return is_end;
    }

    pub fn safe_pop(&mut self) -> String {
        match self.pop() {
            Some(n) => n,
            None => {
                error(format!(
                    "Attempted to call function with too few parameters,\nor call a function using an improper amount of braces: \n\n\"{}\"",
                    self.preserved_program
                ));
                process::exit(1);
            }
        }
    }

    pub fn step(&mut self) {
        let instruction = self.next();

        match instruction {
            Some(n) => {
                if n == "!".to_string() {
                    let argument = &self.safe_pop();
                    let function = &self.safe_pop();
                    self.push_front(split(&call(function, argument)));
                } else if n == "@input".to_string() {
                    self.push(vec![stdin()]);
                } else if n == "@exit".to_string() {
                    process::exit(0);
                } else if n == "@pred".to_string() {
                    let num_a = &self.safe_pop();

                    let my_line = self.preserved_program.clone();

                    // println!("num_a: {}", num_a);
                    // println!("num_b: {}", num_b);

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() - 1, 0);
                    
                    let data = 
                        Evaluator::new(
                            &("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")
                            , &my_line)
                            .eval();
                    self.push(
                        data
                        );
                } else if n == "@succ".to_string() {
                    let num_a = &self.safe_pop();

                    let my_line = self.preserved_program.clone();

                    // println!("num_a: {}", num_a);
                    // println!("num_b: {}", num_b);

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() + 1, 0);
                    
                    let data = 
                        Evaluator::new(
                            &("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")
                            , &my_line)
                            .eval();
                    self.push(
                        data
                        );
                } else if n == "@add".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let my_line = self.preserved_program.clone();

                    // println!("num_a: {}", num_a);
                    // println!("num_b: {}", num_b);

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() + num_b
                            .matches("!")
                            .count(), 0);
                    
                    let data = 
                        Evaluator::new(
                            &("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")
                            , &my_line)
                            .eval();
                    self.push(
                        data
                        );
                } else if n == "@sub".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let my_line = self.preserved_program.clone();

                    // println!("num_a: {}", num_a);
                    // println!("num_b: {}", num_b);

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() - num_b
                            .matches("!")
                            .count(), 0);
                    
                    let data = 
                        Evaluator::new(
                            &("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")
                            , &my_line)
                            .eval();
                    self.push(
                        data
                        );
                } else if n == "@mul".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let my_line = self.preserved_program.clone();

                    // println!("num_a: {}", num_a);
                    // println!("num_b: {}", num_b);

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() * num_b
                            .matches("!")
                            .count(), 0);
                    
                    let data = 
                        Evaluator::new(
                            &("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")
                            , &my_line)
                            .eval();
                    self.push(
                        data
                        );
                } else if n == "@div".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let my_line = self.preserved_program.clone();

                    // println!("num_a: {}", num_a);
                    // println!("num_b: {}", num_b);

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() / num_b
                            .matches("!")
                            .count(), 0);
                    
                    let data = 
                        Evaluator::new(
                            &("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")
                            , &my_line)
                            .eval();
                    self.push(
                        data
                        );
                } else if n == "@eq".to_string() {
                    let arg1 = &self.safe_pop();
                    let arg2 = &self.safe_pop();

                    let result = match arg1 == arg2 {
                        true => "a.b.a",
                        false => "a.b.b",
                    };
                    self.push(vec![result.to_string()]);
                } else if n == "@concat".to_string() {
                    let arg1 = &self.safe_pop();
                    let arg2 = &self.safe_pop();

                    self.push(
                        vec!["none.(".to_string() + &(unfold(&call(arg1, "none")) + &unfold(&call(arg2, "none"))) + ")"]
                    )

                } else if n == "@rec".to_string() {
                    let function = &self.safe_pop();
                    let mut argument = self.safe_pop();
                    loop {
                        let my_line = self.preserved_program.clone();
                        argument =
                            Evaluator::new(&format!("({}) ({}) !", function, argument), &my_line)
                                .eval()
                                .join("");
                        if argument == "@break".to_string() {
                            break;
                        }
                    }
                } else if n == "@eval".to_string() {
                    let mut preprocessor = Preprocessor::new();
                    let arg = &self.safe_pop();
                    self.push(vec![
                        Evaluator::new(&mut preprocessor.process(arg), arg)
                            .eval()
                            .join(""),
                    ]);
                } else if n == "@print".to_string() {
                    print!(
                        "{}",
                        remove_escape_codes(&(self.safe_pop()))
                    );
                } else if n == "@print*".to_string() {
                    print!(
                        "{}",
                        remove_escape_codes(
                            &self.data
                                .clone()
                                .join("")
                                )
                    );
                } else if n == "@println".to_string() {
                    println!(
                        "{}",
                        remove_escape_codes(&(self.safe_pop()))
                    );
                } else if n == "@println*".to_string() {
                    println!(
                        "{}",
                        remove_escape_codes(
                            &self.data
                                .clone()
                                .join("")
                                )
                    );
                } else if n == "@print_pipe".to_string() {
                    let popped = self.safe_pop();
                    print!(
                        "{}",
                        remove_escape_codes(&popped)
                    );
                    self.push(vec![popped]);
                } else if n == "@print_pipe*".to_string() {
                    let popped = self.safe_pop();
                    print!(
                        "{}",
                        remove_escape_codes(
                            &(unfold(&call(&popped, "_")))
                            )
                    );
                    self.push(vec![popped]);
                } else {
                    self.push(split(&n));
                }
            }
            None => {
                process::exit(1);
            }
        };

        if self.logging {
            debug(format!("{:?}", self.data));
        }
    }

    pub fn eval(&mut self) -> Vec<String> {
        while self.data.len() > 0 && !self.end() {
            self.step();
        }
        return self.data.clone();
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        println!("{:?}", self.data);
    }
}
