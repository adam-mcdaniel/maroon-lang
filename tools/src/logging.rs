use std::fmt::*;

// def info(s):
//     print("=( INFO )====>", s)

#[allow(dead_code)]
pub fn info<T: Display>(s: T) {
    println!("=( INFO )====> {}", s);
}

// def debug(s):
//     print("=| DEBUG |===>", s)
#[allow(dead_code)]
pub fn debug<T: Display>(s: T) {
    println!("=| DEBUG |===> {}", s);
}

// def warning(s):
//     print("={ WARNING }=>", s)
#[allow(dead_code)]
pub fn warning<T: Display>(s: T) {
    println!("={{ WARNING }}=> {}", s);
}

// def error(s):
//     print("=[ ERROR ]===>", s)
#[allow(dead_code)]
pub fn error<T: Display>(s: T) {
    println!("=[ ERROR ]===> {}", s);
}
