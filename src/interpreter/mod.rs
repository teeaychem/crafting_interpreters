use std::io::Write;

use crate::{
    ast::statement::{self, Statement, Statements},
    parser::{evaluate::Evaluate, value::ValueError},
};

pub struct Interpreter<'i> {
    d: Box<dyn Write + 'i>,
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
        }
    }

    pub fn interpret(&mut self, statement: &Statement) -> Result<(), ValueError> {
        match statement {
            Statement::Print { e } => {
                let evaluation = e.evaluate()?;
                self.d.write(format!("{evaluation}\n").as_bytes());
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

    #[test]
    fn print() {
        let mut buffer = [0x20; 10];

        let mut scannar = Scanner::default();
        let input = "print 5 + 5; print 5 - 5; ";

        scannar.scan(input);

        let mut parser = Parser::from(scannar);

        parser.parse();

        let mut stream = BufWriter::new(buffer.as_mut());
        // let mut stream = std::io::stdout();

        {
            let mut interpreter = Interpreter::new();

            interpreter.set_destination(&mut stream);

            interpreter.interpret_all(&parser.statements);
        }

        let buffer_string = std::str::from_utf8(&stream.buffer());

        assert_eq!("10\n0\n", buffer_string.expect("Failed to interpret"));
    }
}
