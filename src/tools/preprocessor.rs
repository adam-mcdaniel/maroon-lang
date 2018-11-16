use std::collections::HashMap;
use tools::string_tools::*;
use tools::evaluator::*;

pub struct Preprocessor {
    context: HashMap<String, String>
}


impl Preprocessor {
    pub fn new() -> Self {

        let mut p = Preprocessor {
            context: HashMap::new()
        };

        // p.process("Print = (Print_A.(Print_A @print))");
        // p.process("Println = (Println_A.(Println_A @println))");

        p.process("True = (True_A.(True_B.(True_A)))");
        p.process("False = (False_A.(False_B.(False_B)))");
        p.process("And = (And_P.And_Q.(And_P [And_Q] [And_P]))");
        p.process("Or = (Or_P.Or_Q.(Or_P [Or_P] [Or_Q]))");
        p.process("Not = (Not_P.(Not_P[False][True]))");
        p.process("Nand = Nand_A.Nand_B.(Not[And[Nand_A][Nand_B]])");
        p.process("If = If_P.(If_A.(If_B.(If_P[If_A][If_B])))");
        p.process("Eq = Eq_A.Eq_B.(Eq_A Eq_B @eq)");
        p.process("Eq = Eq_A.Eq_B.(If [Eq[Eq_A][Eq_B]] [True][False])");
        p.process("NotEq = NotEq_A.NotEq_B.(Not[Eq[NotEq_A][NotEq_B]])");
        p.process("Xor = Xor_A.(Xor_B.(And[Or[Xor_A][Xor_B]][Not[Eq[Xor_A][Xor_B]]]))");


        p.process("Pair = Pair_X.Pair_Y.(Pair_Z.(Pair_Z&[Pair_X][Pair_Y]))");
        p.process("Head = First_P.(First_P[True])");
        p.process("Tail = Second_P.(Second_P[False])");
        p.process("Index = Index_P.Index_N.(Head[Index_N[Tail][Index_P]])");

        p.process("ToStr = ToStrA.( none.(ToStrA) )");
        p.process("Put = PutA.(PutA @print)");
        p.process("PutLn = PutLnA.(PutLnA @println)");
        p.process("PutStr = PutStrA.(PutStrA[_] @print*)");
        p.process("PutStrLn = PutStrLnA.(PutStrLnA[_] @println*)");
        p.process("Input = InputA.(ToStr[@input])");

        p.process("Succ = Succ_N.Succ_F.Succ_X.( Succ_F[Succ_N[Succ_F][Succ_X] ] )");
        p.process("Plus = Plus_M.Plus_N.Plus_F.Plus_X.( Plus_M[Plus_F][Plus_N[Plus_F][Plus_X]] )");
        p.process("Mult = Mult_M.Mult_N.Mult_F.Mult_X.( Mult_M[Mult_N[Mult_F]][Mult_X])");
        p.process("0 = False");
        p.process("1 = Succ[0]");


        return p;
    }

    pub fn get_expressions(program_lines: Vec<String>) -> Vec<String> {
        let mut expressions: Vec<String> = vec!["".to_string()];
        let mut skip = 0;

        let mut edited_program = program_lines;
        edited_program.append(&mut vec!["".to_string()]);
        let trimmed_program: Vec<String> = edited_program
                                                .iter()
                                                .map(|x: &String| {x.trim().to_string()})
                                                .collect();


        for trimmed_line in trimmed_program.iter() {
            
            if skip > 0 {
                skip -= 1;
                continue;
            }

            let mut i = expressions.len()-1;

            {
                // trimmed_line.chars().nth(trimmed_line.len()-1)
                let mut s = &mut expressions[i];

                if s.chars().nth(s.len()-1) == Some('[') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
                else if trimmed_line.chars().nth(0) == Some(']') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
                else if s.chars().nth(0) == Some('[') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
                else if trimmed_line.chars().nth(0) == Some('[') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }


                else if s.chars().nth(s.len()-1) == Some('(') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
                else if trimmed_line.chars().nth(0) == Some(')') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
                else if s.chars().nth(0) == Some('(') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
                else if trimmed_line.chars().nth(0) == Some('(') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }


                else if s.chars().nth(0) == Some('.') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
                else if trimmed_line.chars().nth(0) == Some('.') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
            }
            
            expressions.append(&mut vec![trimmed_line.to_string()]);
        }

        // println!("{:?}", expressions);
        return expressions;
    }

    pub fn to_primitive_call(&mut self, line: &str) -> String {
        line
            .replace("[", " ")
            .replace("]", " !")
    }

    #[allow(dead_code)]
    pub fn replace_strings(&mut self, line: &str) -> String {
        // line
        //     .replace("[", " ")
        //     .replace("]", " !")
        let mut result: String = "".to_string();

        let mut quote_count = 0;
        for c in line.chars() {
            match c {
                '"' => {
                    
                    if quote_count == 1 {
                        result += ")";
                        // close quote
                        quote_count = 0;
                    } else {
                        result += "none.(";
                        quote_count += 1;
                    }

                },

                ' ' => {
                    if quote_count == 1 {
                        result += " \\_ ";
                    } else {
                        result += " ";
                    }
                },

                some_char => {
                    result += &some_char.to_string();
                }
            }
        }
        // println!("string: {}", line);
        return result;
    }

    pub fn bind(&mut self, line: &str) -> String {
        if line.contains('=') {
            // println!("{:?}", self.context);
            let line_vec = split(line);
            // let value = Evaluator::new(&self.process(&line_vec[line_vec.len()-1].clone())).eval().join("");
            // let value = Evaluator::new(&self.process(&line_vec[2..].join(" ").clone())).eval().join("");
            let value = "(".to_owned() + &Evaluator::new(&self.process(&line[find(&line, "=")+1..]), &line[find(&line, "=")+1..]).eval().join(" ") + ")"; 
            self.context.insert(
                line_vec[0].clone(),
                value.to_string(),
                );
            // println!("{:?}", self.context);

            return "".to_string();
        } else {
            // let mut line_vec = split(line);

            // line_vec = line_vec
            //     .iter()
            //     .map(move |x| 
            //     {
            //         println!("\tTOKEN: {}", x);
            //         match self.context.get(x) {
            //             Some(n) => n.to_string(),
            //             None => x.to_string()
            //         }
            //     }).collect();
            let mut replaced_line: String = line.to_string();
            let mut replacement: &str;

            for key in self.context.keys() {
                replacement = match self.context.get(key).to_owned() {
                    Some(n) => n,
                    None => ""
                };
                replaced_line = call(&(key.to_owned() + ".(" + &replaced_line + ")"), replacement);
                // replaced_line = unfold(&replaced_line);
            }

            // println!("{:?}", line_vec);
            return replaced_line.to_string();
        }
    }

    pub fn process(&mut self, line: &str) -> String {
        let line = &self.replace_strings(line);
        let calls_checked = &self.to_primitive_call(
                line  
            );

        let bounded = self.bind(calls_checked);
        // println!("{} -> {}", line, bounded);
        return bounded;
    }
}