#![allow(dead_code)]

use std::io::BufRead;

use scanner::Scanner;

mod scanner;
mod tokens;
mod expression;

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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Location {
    col: usize,
    line: usize,
}

impl Default for Location {
    fn default() -> Self {
        Location { col: 0, line: 0 }
    }
}

impl Location {
    pub fn newline(&mut self) {
        self.line += 1;
        self.col = 0;
    }
}
