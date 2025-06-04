#![allow(dead_code, unused, clippy::derivable_impls)]

use std::io::BufRead;

pub use interpreter::parser::Parser;
pub use interpreter::scanner::Scanner;
pub use interpreter::Interpreter;

pub mod interpreter;
