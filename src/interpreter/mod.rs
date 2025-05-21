use std::{collections::HashMap, io::Write};

use crate::{
    ast::{
        expression::Expression,
        literal::Literal,
        statement::{self, Statement, Statements},
    },
    parser::value::{self, Value, ValueError},
};

pub mod evaluate;

pub struct Interpreter<'i> {
    d: Box<dyn Write + 'i>,
    e: HashMap<String, Value>,
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
            Statement::Expression { e } => {
                self.evaluate(e)?;
            }

            
            Statement::Print { e } => {
                let evaluation = self.evaluate(e)?;

                self.d.write(format!("{evaluation}\n").as_bytes());
            }

            Statement::Declaration { id: name, assignment } => {
                let name = self.get_identifier(name)?;

                if self.e.get(&name).is_some() {
                    return Err(ValueError::Redeclaration);
                } else {
                    let assignment = self.evaluate(assignment)?;
                    self.e.insert(name, assignment.clone());
                }
            }            

            _ => todo!("Inpereter todo: {statement:?}"),
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
    }

    #[test]
    fn print_string() {
        let input = "print \"print\";";
        let output = "print";
        test_io(input, output);
    }

    #[test]
    fn declaration() {
        let input = "var test = \"testing\"; test = \"testing again\"; print test;";
        let output = "testing again";
        test_io(input, output);
    }
}
