#![allow(dead_code, unused)]

use std::io::{BufRead, Read, Write};

use loxy_lib::interpreter::{Base, TreeWalker};

fn main() {
    let stdin = std::io::stdin();

    let mut walker = TreeWalker::default();
    let mut base = Base::default();

    let mut statement_count = 0;

    let mut buffer = String::with_capacity(512);

    loop {
        let read_result = stdin.lock().read_to_string(&mut buffer);

        match read_result {
            Ok(_) => {}
            Err(e) => panic!("!{e:?}"),
        }

        match walker.scan(&buffer) {
            Ok(()) => {}
            Err(e) => {
                walker.handle_stumble(&e);
                std::process::exit(-1);
            }
        };

        let fresh_statements = match walker.parse() {
            Ok(count) => count,
            Err(e) => {
                walker.handle_stumble(&e);
                std::process::exit(-2);
            }
        };

        buffer.clear();

        if 0 < fresh_statements {
            print!("\x1b[2K\r"); // Clear the current line using escape codes

            for idx in statement_count..statement_count + fresh_statements {
                match walker.interpret_index(&mut base, idx) {
                    Ok(()) => {}
                    Err(e) => {
                        walker.handle_stumble(&e);
                        std::process::exit(-3);
                    }
                };
            }

            statement_count += fresh_statements;
        }
    }
}
