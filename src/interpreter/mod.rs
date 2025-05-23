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

#[cfg(test)]
mod tests;

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
