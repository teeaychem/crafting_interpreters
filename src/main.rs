#![allow(dead_code, unused)]

use std::io::BufRead;

use scanner::Scanner;

mod ast;
mod location;
mod parser;
mod scanner;

fn main() {
    println!("Hello, world!");

    let _scanner = Scanner::default();

    let stdin = std::io::stdin();
    println!("x");
    let mut buf = String::with_capacity(80);

    while stdin.lock().read_line(&mut buf).is_ok() {
        let mut chars = buf.chars();
        chars.next();
        println!("{:?}", chars.collect::<Vec<_>>());
        buf.clear();
    }

    println!("Exit");
}

