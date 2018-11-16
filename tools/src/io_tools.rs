use std::fs::File;
use std::io;
use std::io::*;

use string_tools::*;

pub fn stdin() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line");

    let line = input[..input.len() - 1].to_string();

    let mut result = "".to_string();

    for c in line.chars() {
        match c {
            ' ' => {
                result += " \\_ ";
            }

            '\\' => {
                result += "\\\\";
            }
            '(' => {
                result += "\\lp";
            }
            ')' => {
                result += "\\rp";
            }
            '[' => {
                result += "\\lb";
            }
            ']' => {
                result += "\\rb";
            }
            '!' => {
                result += "\\x";
            }

            some_char => {
                result += &some_char.to_string();
            }
        }
    }
    return result;
}

#[allow(dead_code)]
pub fn read(file_name: &str) -> String {
    let mut file = BufReader::new(File::open(file_name).unwrap());

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    return s;
}

pub fn readlines(file_name: &str) -> Vec<String> {
    let file = BufReader::new(File::open(file_name).unwrap());
    let lines: Vec<_> = file
        .lines()
        .map(|line| remove_comments(&line.unwrap()))
        .collect();

    return lines;
}
