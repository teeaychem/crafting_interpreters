use std::{collections::HashMap, io::Write};

use environment::Env;

use crate::{
    ast::{
        expression::Expression,
        literal::Literal,
        statement::{self, Statement, Statements},
    },
    parser::value::{self, Value, ValueError},
};

pub mod environment;
pub mod evaluate;

pub struct Interpreter<'i> {
    out: Box<dyn Write + 'i>,
    env: Env,
}

impl<'i> Interpreter<'i> {
    pub fn set_destination<T: Write + 'i>(&mut self, d: T) {
        self.out = Box::new(d)
    }
}

impl Interpreter<'_> {
    pub fn new() -> Self {
        Interpreter {
            out: Box::new(std::io::stdout()),
            env: Env::default(),
        }
    }

    pub fn interpret(&mut self, statement: &Statement) -> Result<(), ValueError> {
        match statement {
            Statement::Expression { e } => {
                self.evaluate(e)?;
            }

            Statement::Print { e } => {
                let evaluation = self.evaluate(e)?;

                self.out.write(format!("{evaluation}\n").as_bytes());
            }

            Statement::Declaration { id, e } => {
                let id = self.get_identifier(id)?;

                let assignment = self.evaluate(e)?;

                self.env.insert(id, &assignment);
            }

            Statement::BlockEnter => {
                self.env.narrow();
            }

            Statement::BlockExit => {
                self.env.relax();
            }

            Statement::Conditional { condition, yes, no } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.interpret(yes);
                } else if let Some(no) = no {
                    self.interpret(no);
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

            match interpreter.interpret_all(parser.statements()) {
                Ok(_) => {}

                Err(e) => panic!("Interpretation error: {e:?}"),
            };
        }

        let buffer_string = std::str::from_utf8(&stream.buffer());

        assert_eq!(buffer_string.expect("Failed to interpret").trim(), output);
    }

    #[test]
    fn print() {
        test_io("print 5 + 5; print 5 - 5; ", "10\n0");

        test_io("print true;", "true");

        test_io("print !true;", "false");
    }

    #[test]
    fn print_string() {
        let input = "print \"print\";";

        test_io(input, "print");
    }

    #[test]
    fn declaration() {
        let input = "var test = \"testing\"; test = \"testing again\"; print test;";

        test_io(input, "testing again");
    }

    #[test]
    fn nested() {
        let input = "
var a = \"a\";
var b = \"b\";
a = b = \"c\";
print a;
";

        test_io(input, "c");
    }

    #[test]
    fn print_addition() {
        let input = "
var a = 3;
var b = 3;
print (a * b) / (a + b);";
        test_io(input, "1.5");
    }

    #[test]
    fn var_nil() {
        let input = "
var a;
print a;";
        test_io(input, "nil");
    }

    #[test]
    fn block_basic() {
        let input = "
var a;
print a;
{
  a = 1;
  print a;
}
print a;
";
        test_io(input, "nil\n1\nnil");
    }

    #[test]
    fn block_nested() {
        let input = "
var a = \"global a\";
var b = \"global b\";
var c = \"global c\";
{
  var a = \"outer a\";
  var b = \"outer b\";
  {
    var a = \"inner a\";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;
";
        test_io(
            input,
            "inner a
outer b
global c
outer a
outer b
global c
global a
global b
global c",
        );
    }

    #[test]
    fn challenge() {
        let input = "
var a = 1;
{
var a = a + 2;
print a;
}";

        test_io(input, "3");
    }

    #[test]
    fn conditional() {
        let input = "
if (false != true) print \"ok\";";
        test_io(input, "ok");

        let input = "
if (false == true) print \"nok\";";
        test_io(input, "");

        let input = "
if (false == true) print \"nok\"; else print \"ok\";";
        test_io(input, "ok");
    }
}
