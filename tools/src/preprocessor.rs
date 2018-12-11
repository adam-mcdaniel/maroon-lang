use evaluator::*;
use std::path::Path;
use std::collections::HashMap;
use string_tools::*;
use io_tools::*;
use std::env;

pub struct Preprocessor {
    context: HashMap<String, String>,
}

impl Preprocessor {
    pub fn new() -> Self {
        let mut p = Preprocessor {
            context: HashMap::new(),
        };

        p.process("Import = Import_A.(Import_A @import)");
        p.process("Concat = Concat_A.Concat_B.(Concat_B Concat_A @concat)");
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
        p.process("ToVal = ToValA.( ToValA[none] )");

        p.process("Newln = NewlnA.(\\_ @println NewlnA)");
        p.process("Pipe = PipeA.(PipeA @print_pipe)");
        p.process("Pipeln = PipelnA.(PipelnA @print_pipe \\_ @println)");
        p.process("PipeStr = PipeStrA.(PipeStrA @print_pipe*)");
        p.process("PipeStrln = PipeStrlnA.(PipeStrlnA @print_pipe* \\_ @println)");
        p.process("Input = InputA.(ToStr[@input])");

        p.process("Put = Pipe");
        p.process("Putln = Pipeln");
        p.process("PutStr = PipeStr");
        p.process("PutStrln = PipeStrln");


        p.process("PipeFn = Pipe_function.Pipe_x.(v.()[Pipe_function[Pipe_x]] Pipe_x)");

        p.process("Rec = Rec_Function.Rec_Argument.(Rec_Argument Rec_Function @rec)");
        p.process("Break = Break_A.(@break)");

        p.process("Succ = Succ_N.Succ_F.Succ_X.( Succ_F[Succ_N&[Succ_F&][Succ_X&] ] )");
        p.process("Add = Plus_M.Plus_N.Plus_F.Plus_X.( Plus_M[Plus_F][Plus_N[Plus_F][Plus_X]] )");

        p.process("Pred = Pred_N.(Pred_N @pred)");

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
            .map(|x: &String| x.trim().to_string())
            .collect();

        for trimmed_line in trimmed_program.iter() {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            let mut i = expressions.len() - 1;

            {
                // trimmed_line.chars().nth(trimmed_line.len()-1)
                let mut s = &mut expressions[i];

                if s.chars().nth(s.len() - 1) == Some('[') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if trimmed_line.chars().nth(0) == Some(']') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if s.chars().nth(0) == Some('[') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if trimmed_line.chars().nth(0) == Some('[') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if s.chars().nth(s.len() - 1) == Some('(') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if trimmed_line.chars().nth(0) == Some(')') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if s.chars().nth(0) == Some('(') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if trimmed_line.chars().nth(0) == Some('(') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if s.chars().nth(0) == Some('.') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                } else if trimmed_line.chars().nth(0) == Some('.') {
                    *s = s.to_owned() + trimmed_line;
                    continue;
                }
            }

            expressions.append(&mut vec![trimmed_line.to_string()]);
        }
        return expressions;
    }

    pub fn to_primitive_call(&mut self, line: &str) -> String {
        line.replace("[", " ").replace("]", " !")
    }

    #[allow(dead_code)]
    pub fn replace_strings(&mut self, line: &str) -> String {
        let mut result: String = "".to_string();

        let mut quote_count = 0;
        for c in line.chars() {
            match c {
                '"' => {
                    if quote_count == 1 {
                        result += "\\.)";
                        // close quote
                        quote_count = 0;
                    } else {
                        result += "none.(\\.";
                        quote_count += 1;
                    }
                }

                ' ' => {
                    if quote_count == 1 {
                        result += "\\. \\_ \\.";
                    } else {
                        result += " ";
                    }
                }

                '=' => {
                    if quote_count == 1 {
                        result += "\\.\\e\\.";
                    } else {
                        result += "=";
                    }
                }
                '!' => {
                    if quote_count == 1 {
                        result += "\\.\\x\\.";
                    } else {
                        result += "!";
                    }
                }
                '(' => {
                    if quote_count == 1 {
                        result += "\\.\\lp\\.";
                    } else {
                        result += "(";
                    }
                }
                ')' => {
                    if quote_count == 1 {
                        result += "\\.\\rp\\.";
                    } else {
                        result += ")";
                    }
                }
                '[' => {
                    if quote_count == 1 {
                        result += "\\.\\lb\\.";
                    } else {
                        result += "[";
                    }
                }
                ']' => {
                    if quote_count == 1 {
                        result += "\\.\\rb\\.";
                    } else {
                        result += "]";
                    }
                }
                '\\' => {
                    if quote_count == 1 {
                        result += "\\.\\\\\\.";
                    } else {
                        result += "\\";
                    }
                }
                '@' => {
                    if quote_count == 1 {
                        result += "\\.@\\.";
                    } else {
                        result += "@";
                    }
                }

                some_char => {
                    result += &some_char.to_string();
                }
            }
        }
        return result;
    }

    // maps left of = to right of =
    pub fn bind(&mut self, line: &str) -> String {
        if line.contains('=') {
            let line_vec = split(line);
            let value = "(".to_owned()
                + &Evaluator::new(
                    &self.process(&line[find(&line, "=") + 1..]),
                    &line[find(&line, "=") + 1..],
                ).eval()
                .join(" ")
                + ")";
            self.context.insert(line_vec[0].clone(), value.to_string());

            return "".to_string();
        } else {
            let mut replaced_line: String = line.to_string();
            let mut replacement: &str;

            for key in self.context.keys() {
                replacement = match self.context.get(key).to_owned() {
                    Some(n) => n,
                    None => "",
                };
                replaced_line = call(&(key.to_owned() + ".(" + &replaced_line + ")"), replacement);
            }

            return replaced_line.to_string();
        }
    }

    pub fn import(&mut self, line: &str) {
        let args: Vec<String> = env::args().collect();

        if line.contains(&"@import".to_string()) {
            let evaluated_expr = Evaluator::new(
                                                line,
                                                line,
                                                ).eval();
            if evaluated_expr.len() == 2 && evaluated_expr[1] == "@import" {

                let name_in_stack = Evaluator::new(
                                        &self.process(
                                            &("ToVal[".to_owned() + &evaluated_expr[0] + "]")
                                            ),
                                        line,
                                        ).eval();

                let name = remove_escape_codes(&name_in_stack.join(""));

                for line in Preprocessor::get_expressions(readlines(&(Path::new(&args[1]).parent().unwrap().to_str().unwrap().to_owned() + "/" + &name + ".m"))) {
                    Evaluator::new(&self.process(&line), &line).eval();
                }
            }
        }
    }

    pub fn process(&mut self, line: &str) -> String {
        let line = &self.replace_strings(line);

        let calls_checked = &self.to_primitive_call(line);

        let bounded = self.bind(calls_checked);
        self.import(&bounded);

        // println!("{} -> {}", line, bounded);
        return bounded;
    }
}
