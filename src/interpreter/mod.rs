use std::{collections::HashMap, io::Write};

use crate::{
    ast::{
        expression::Expression,
        statement::{self, Statement, Statements},
    },
    parser::value::{self, ValueError},
};

pub mod evaluate;

pub struct Interpreter<'i> {
    d: Box<dyn Write + 'i>,
    e: HashMap<String, Expression>,
}

impl<'i> Interpreter<'i> {
    pub fn set_destination<T: Write + 'i>(&mut self, d: T) {
        self.d = Box::new(d)
    }
}

impl Interpreter<'_> {
    pub fn new() -> Self {
        Interpreter {
            d: Box::new(std::io::stdout()),
            e: HashMap::default(),
        }
    }

    pub fn interpret(&mut self, statement: &Statement) -> Result<(), ValueError> {
        println!("Interpreting: {statement:?}");
        match statement {
            Statement::Print { e } => {
                let evaluation = self.evaluate(e)?;

                self.d.write(format!("{evaluation}\n").as_bytes());
            }

            Statement::Declaration { name, assignment } => {
                if let Some(value) = self.e.get_mut(name) {
                    *value = assignment.clone();
                } else {
                    self.e.insert(name.to_owned(), assignment.clone());
                };
            }

            _ => todo!("{statement:?}"),
        }

        Ok(())
    }

    pub fn interpret_all(&mut self, statements: &Statements) -> Result<(), ValueError> {
        for statement in statements {
            self.interpret(statement)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use crate::{parser::Parser, scanner::Scanner};

    use super::*;

    fn test_io(input: &str, output: &str) {
        let mut scanner = Scanner::default();
        let mut parser = Parser::default();

        scanner.scan(input);
        parser.consume_scanner(scanner);
        match parser.parse() {
            Ok(_) => {}

            Err(e) => {
                println!("Parser failure: {e:?}");
                dbg!(&parser);
                panic!();
            }
        };
        

        let mut buffer = Vec::with_capacity(output.len());
        let mut stream = BufWriter::new(&mut buffer);

        {
            let mut interpreter = Interpreter::new();

            interpreter.set_destination(&mut stream);

            interpreter.interpret_all(parser.statements());
        }

        let buffer_string = std::str::from_utf8(&stream.buffer());

        assert_eq!(output, buffer_string.expect("Failed to interpret").trim());
    }

    #[test]
    fn print() {
        let input = "print 5 + 5; print 5 - 5; ";
        let output = "10\n0";
        test_io(input, output);

        let input = "print true;";
        let output = "true";
        test_io(input, output);

        let input = "print !true;";
        let output = "false";
        test_io(input, output);

        let input = "print \"print\";";
        let output = "print";
        test_io(input, output);
    }

    #[test]
    fn declaration() {
        let input = "var test = \"test\"; print test;";
        let output = "test";
        test_io(input, output);
    }
}
