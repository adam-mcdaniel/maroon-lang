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


pub fn remove_comments(line: &str) -> String {
    // returns an empty string if line is a comment
    if line.len() > 2 {
        if &line[..2] == "//" {
            return "".to_string();
        }
    }
    return line.to_string();
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

// return the index of the first instance of a substring in a string
pub fn find(string: &str, substring: &str) -> usize {
    let i = match (&string).find(substring) {
        Some(n) => n,
        None => 0 as usize,
    };
    return i;
}

// returns a called function
// takes a lambda function as a string
// replaces all instances of the parameter with the applied argument
pub fn call(function: &str, arg: &str) -> String {

    let parameter_name = &function[..find(&function, ".")];
    let mut result = (&function[find(&function, ".")+1..]).to_string();

    let mut token = "".to_string();
    let mut n: i32 = 0;

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
            if !([' ', '(', ')', '.'].contains(&c)) {
                token.push(c);
            } else {

                if token == parameter_name.to_string() && c != '.' {
                    for _ in 0..(token.len()) {
                        result = remove(&result, (n as usize)-token.len()-1);
                    }
                    result = insert(&result, &arg, (n as usize)-token.len()-1);
                    n += ((arg.len()-1) as i32 - (token.len() as i32))+ 1;


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

// class Grouper:
//     def unfold(self, s):
//         # remove parentheses from a substring
//         if "(" in s:
//             pars = 1
//             for i, char in enumerate(s[s.find("(", 1)+1:]):
//                 # print(char, pars)
//                 if char == "(": pars += 1
//                 elif char == ")": pars -= 1
                
//                 if pars == 0: break
                    

//             s = remove(s, i + s.find("(", 1)+1)
//             s = remove(s, s.find("(", 1))

//         return s

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
//     def between_pars(self, s, i):

//         in_pars = True
//         begin = 0

//         pars = 0
//         for j, char in enumerate(s):
//             # print(char, pars)

//             if char == "(":
//                 if pars == 0:
//                     begin = j
//                 pars += 1

//             elif char == ")": pars -= 1
            
//             if pars > 0:
//                 in_pars = True

//             if pars == 0:
//                 if begin < i and i < j and in_pars:
//                     return True

//                 in_pars = False

//         return False

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

//     def split(self, s):
//         head = ""
//         tail = ""

//         for i, char in enumerate(s):
//             if i == len(s)-1:
//                 head = s
//                 break

//             elif char == " " and not self.between_pars(s, i):
//                 head = s[:i]
//                 tail = s[i+1:]
//                 break

//             elif char == "(" and i == 0:

//                 pars = 0
//                 for j, par_test in enumerate(s):
//                     if par_test == "(": pars += 1
//                     elif par_test == ")": pars -= 1
                    
//                     if pars == 0: break
                
//                 head = s[:j+1]
//                 tail = s[j+1:]

//                 head = self.split(self.unfold(" " + head + " ").strip())
                
//                 if len(head) == 1:
//                     head = ' '.join(head)

//                 break



//         if tail == "":
//             if not type(head) == list:
//                 return [head]
//             return head

//         if head == "":
//             return self.split(tail)
        
//         if not type(head) == list:
//             return [head] + self.split(tail)
//         return head + self.split(tail)

