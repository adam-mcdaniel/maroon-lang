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
                "@println_raw".to_string(),
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
                self.error("Attempted to call function with too few parameters,\nor call a function using an improper amount of braces")
            }
        }
    }

    pub fn error(&self, message: &str) -> String {
        error(format!(
            "{}: \n\n\"{}\"",
            message,
            self.preserved_program
        ));
        process::exit(1);
        // return "".to_string()
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
                } else if n == "@to_fun".to_string() {
                    let num_a = &self.safe_pop();

                    let str_num = unfold(&call(&remove_escape_codes(num_a), "none"));
                    let num = match str_num.parse::<i32>() {
                        Ok(n) => n,
                        Err(_) => 0
                    } as usize;

                    self.push(
                        vec![("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")]
                        );
                } else if n == "@num".to_string() {
                    let num_a = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count(), 0);

                    self.push(
                        vec!["none.(\\.".to_owned() + &num.to_string()+"\\.)"]
                        );
                } else if n == "@index_string".to_string() {
                    let num_a = &self.safe_pop();
                    let string_a = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count(), 0);

                    let inserted_value = &(match &from_maroon_string(string_a).chars().nth(num) {
                            Some(n) => n,
                            None => {
                                self.error("Index not in range");
                                &' '
                            }
                        }).to_string();

                    self.push(
                        vec!["none.(\\.".to_owned() + inserted_value +"\\.)"]
                        );
                } else if n == "@in_string".to_string() {
                    let string_a = &from_maroon_string(&self.safe_pop());
                    let string_b = &from_maroon_string(&self.safe_pop());

                    self.push(vec![match string_a.contains(string_b) {
                        true => "a.b.a".to_string(),
                        false => "a.b.b".to_string()
                        }]);
                } else if n == "@replace_string".to_string() {
                    let string_a = &self.safe_pop();
                    let string_b = &remove_escape_codes(&self.safe_pop());
                    let string_c = &remove_escape_codes(&self.safe_pop());
                    // println!("{} {} {}", string_a, string_b, string_c);
                    self.push(vec![
                        "none.(\\.".to_owned() + &get_inside_string(&string_a).replace(
                            &get_inside_string(&remove_escape_codes(&string_b)),
                            &get_inside_string(&remove_escape_codes(&string_c)),
                        ) + "\\.)"
                        ]);
                } else if n == "@range_string".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();
                    let string_a = &self.safe_pop();

                    let num_first = cmp::max(num_a
                        .matches("!")
                        .count(), 0);
                    let num_last = cmp::max(num_b
                        .matches("!")
                        .count(), 0);
                    if &from_maroon_string(string_a).len() < &num_last {
                        self.error("Range out of bounds");
                    }
                    self.push(
                        vec!["none.(\\.".to_owned() + &from_maroon_string(string_a)[num_first..num_last] + "\\.)"]
                        );
                } else if n == "@pred".to_string() {
                    let num_a = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() - 1, 0);
                    
                    self.push(
                        vec![("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")]
                        );
                } else if n == "@succ".to_string() {
                    let num_a = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() + 1, 0);

                    self.push(
                        vec![("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")]
                        );
                } else if n == "@add".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() + num_b
                            .matches("!")
                            .count(), 0);
                    
                    self.push(
                        vec![("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")]
                        );
                } else if n == "@sub".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() as i32 - num_b
                            .matches("!")
                            .count() as i32, 0) as usize;
                    
                    self.push(
                        vec![("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")]
                        );
                } else if n == "@mul".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() * num_b
                            .matches("!")
                            .count(), 0);
                    
                    self.push(
                        vec![("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")]
                        );
                } else if n == "@div".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() as i32 / num_b
                            .matches("!")
                            .count() as i32, 0) as usize;
                    
                    self.push(
                        vec![("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")]
                        );
                } else if n == "@mod".to_string() {
                    let num_a = &self.safe_pop();
                    let num_b = &self.safe_pop();

                    let num = cmp::max(num_a
                        .matches("!")
                        .count() % num_b
                            .matches("!")
                            .count(), 0);
                    
                    self.push(
                        vec![("F.X.(".to_owned() + &" F".repeat(num) + &" X" + &" !".repeat(num) + &")")]
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
                    let my_line = self.preserved_program.clone();

                    // println!("{} none !", preprocessor.process(&Preprocessor::get_expressions(vec![remove_escape_codes(&arg)]).join(" ")));
                    self.push(vec![
                        Evaluator::new(&format!("{} none !", preprocessor.process(&Preprocessor::get_expressions(vec![remove_escape_codes(&arg)]).join(" "))), &my_line)
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
                } else if n == "@println_raw".to_string() {
                    println!(
                        "{}",
                        &(self.safe_pop())
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
