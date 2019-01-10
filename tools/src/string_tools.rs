#[allow(unused_imports)]
use logging::*;

pub fn remove_escape_codes(line: &str) -> String {
    line.replace(" ","")
        .replace("\\rp", ")")
        .replace("\\lp", "(")
        .replace("\\rb", "]")
        .replace("\\lb", "[")
        .replace("\\x", "!")
        .replace("\\e", "=")
        .replace("\\_", " ")
        .replace("\\\\", "\\\\.")
        .replace("@", "\\.@")
        .replace("\\.", "")
}

pub fn add_escape_codes(line: &str) -> String {
    line.replace("\\", "\\.\\\\\\.")
        .replace(")", "\\.\\rp\\.")    
        .replace("(", "\\.\\lp\\.")
        .replace("]", "\\.\\rb\\.")
        .replace("[", "\\.\\lb\\.")
        .replace("!", "\\.\\x\\.")
        .replace("=", "\\.\\e\\.")
        .replace(" ", "\\. \\_ \\.")
        .replace("@", "\\.@\\.")
}


pub fn from_maroon_string(string: &str) -> String {
    let result = remove_escape_codes(&string[6..string.len()-1]);
    return result;
}


// remove none.() from string
pub fn get_inside_string(string: &str) -> String {
    return string[6..string.len()-1].to_string();
}

// function used to insert one string into another string at a specific index.
pub fn insert(string: &str, str_to_insert: &str, index: usize) -> String {
    let s = string[..index].to_owned() + str_to_insert + &string[index..];
    s
}



// remove a character from a string at a specific index
pub fn remove(string: &str, index: usize) -> String {
    string[..index].to_owned() + &string[index+1..]
}


// remove whitespace
pub fn remove_whitespace(string: &str) -> String {
    // string[..index].to_owned() + &string[index+1..]
    let mut s = "".to_string();

    for c in string.chars() {
        if c != ' ' && c != '\t' {
            s.push(c);
        }
    }

    return s;
}

// return the index of the first instance of a substring in a string
pub fn find(string: &str, substring: &str) -> usize {
    let i = match (&string).find(substring) {
        Some(n) => n,
        None => 0 as usize,
    };
    return i;
}


pub fn remove_comments(line: &str) -> String {
    // removes comments from string


    // if line.len() > 2 {
    //     if &line[..2] == "//" {
    //         return "".to_string();
    //     }
    // }
    // return line.to_string();
    if line.contains("//") {
        return line[..find(&line, "//")].to_string();
    }

    return line.to_string();
}



// returns a called function
// takes a lambda function as a string
// replaces all instances of the parameter with the applied argument
pub fn call(function: &str, arg: &str) -> String {
    // debug(format!("CALL: {}[{}]", function, arg));
    let parameter_name = &function[..find(&function, ".")];
    let mut result = (&function[find(&function, ".")+1..]).to_string();

    let mut token = "".to_string();
    let mut n: i32 = 0;
    
    let mut reached_space = false;
    let mut new_scope = false;
    let mut parentheses = 0;

    let mut first_mutable = false;
    loop {

        // if the index of the current character in the string is greater
        // or equal to the length of the string, break the loop and return
        if n as usize >= result.len() {
            break;
        }

        // iterate over each token and check if that token is the parameter
        // if that token is the parameter, replace it.
        for c in (result.to_owned() + " ").chars() {
            n += 1;


            if new_scope {
                if c == '(' {
                    parentheses += 1;
                } else if c == ')' {
                    parentheses -= 1;
                } else if c == ' ' {
                    reached_space = true;
                }

                if parentheses < 1 && reached_space {
                    reached_space = false;
                    new_scope = false;
                    parentheses = 0;
                } else {
                    continue;
                }
            }

            if !([' ', '(', ')', '.'].contains(&c)) {
                token.push(c);
            } else {
                if token == parameter_name.to_string() && c != '.' {
                    for _ in 0..(token.len()) {
                        result = remove(&result, (n as usize)-token.len()-1);
                    }
                    result = insert(&result, &arg, (n as usize)-token.len()-1);
                    n += ((arg.len()-1) as i32 - (token.len() as i32))+ 1;
                } else if token == parameter_name.to_string() && c == '.' {
                    new_scope = true;
                    reached_space = false;
                    parentheses = 0;
                } else if token == parameter_name.to_string() + "&" && !first_mutable {
                    first_mutable = true;
                    result = remove(&result, n as usize - 2 as usize);
                    n -= 1;
                    token = token[..token.len()-1].to_string();

                    for _ in 0..(token.len()) {
                        result = remove(&result, (n as usize)-token.len()-1);
                    }
                    result = insert(&result, &arg, (n as usize)-token.len()-1);
                    n += ((arg.len()-1) as i32 - (token.len() as i32))+ 1;
                }
                

                token = "".to_string();
            }
        }
    }

    // println!("{}({}) -> {}", function, arg, result);

    return result
}



// Grouper
pub fn unfold(s: &str) -> String {
    // remove parentheses from a substring
    let mut result: String = "".to_string();
    let mut n = 0;

    if s.contains("(") {
        let mut pars = 1;
        for (i, c) in s[find(&s, "(")..].chars().enumerate() {
            match c{
                '(' => pars += 1,
                ')' => pars -= 1,
                _ => {}
            } 

            if pars == 0 { break; }

            n = i;
        }
        
        result = remove(s, n + find(s, "("));
        result = remove(&result, find(&result, "("));
    }

    return result;
}


pub fn between_pars(s: &str, i: usize) -> bool {
    let mut in_pars = true;
    let mut begin = 0;

    let mut pars = 0;

    for (j, c) in s.chars().enumerate() {
        if c == '(' {
            if pars == 0 {
                begin = j;
            }
            pars += 1;
        } else if  c == ')' {
            pars -= 1;
        }

        if pars > 0 {
            in_pars = true;
        }

        if pars == 0 {
            if begin < i && i < j && in_pars {
                return true;
            }
            in_pars = false;
        }
    }
    return false;
}


#[allow(unused_assignments)]
pub fn split(s: &str) -> Vec<String> {
    let mut head_list: Vec<String> = vec![];
    let mut head_string = "";

    let mut tail = "";

    let mut pars = 0;

    for (i, c) in s.chars().enumerate() {
        if i == s.len()-1 {
            head_list = vec![s.to_string()];
            break;
        } else if c == ' ' && !between_pars(&s, i) {
            head_list = vec![s[..i].to_string()];
            tail = &s[i+1..];
            break;
        } else if c == '(' && i == 0 {
            pars = 0;
            for (j, par_test) in s.chars().enumerate() {
                if par_test == '(' { pars += 1; }
                else if par_test == ')' { pars -= 1; }

                if pars == 0 {
                    head_string = &s[..j];
                    tail = &s[j+1..];
                    break;
                }
            }

            head_list = split(unfold(
                &(" ".to_owned() + head_string + " ")
                ).trim());

            break;
        }
    }

    if tail == "" {
        return head_list;
    }

    else if head_list.len() == 0 {
        return split(tail);
    } 

    else {
        head_list.append(&mut split(tail));
        return head_list
            .iter()
            .filter(|x| x != &"")
            .cloned()
            .collect()
    }
}
