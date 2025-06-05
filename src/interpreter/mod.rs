use std::{collections::HashMap, io::Write};

use std::io::BufRead;

pub mod ast;
pub mod location;
pub mod parser;
pub mod scanner;

use environment::Env;

use crate::interpreter::{
    ast::{
        expression::Expression,
        literal::Literal,
        statement::{self, Statement, Statements},
    },
    parser::value::{self, Value, ValueError},
};

pub mod environment;
pub mod evaluate;

pub struct Interpreter {}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {}
    }
}

impl Interpreter {
    pub fn interpret<W: Write>(
        &self,
        statement: &Statement,
        env: &mut Env,
        out: &mut W,
    ) -> Result<(), ValueError> {
        match statement {
            Statement::Expression { e } => {
                self.evaluate(e, env)?;
            }

            Statement::Print { e } => {
                let evaluation = self.evaluate(e, env)?;

                unsafe {
                    out.write(format!("{evaluation}\n").as_bytes());
                }
            }

            Statement::Declaration { id, e } => {
                let id = self.get_identifier(id)?;

                let assignment = self.evaluate(e, env)?;

                env.insert(id, assignment);
            }

            Statement::Block { statements } => {
                env.narrow();

                for statement in statements {
                    self.interpret(statement, env, out);
                }

                env.relax();
            }

            Statement::Conditional {
                condition,
                case_if: yes,
                case_else: no,
            } => {
                if self.evaluate(condition, env)?.is_truthy() {
                    self.interpret(yes, env, out);
                } else if let Some(no) = no {
                    self.interpret(no, env, out);
                }
            }

            Statement::While { condition, body } => {
                while self.evaluate(condition, env)?.is_truthy() {
                    self.interpret(body, env, out);
                }
            }

            _ => todo!("Inpereter todo: {statement:?}"),
        }

        Ok(())
    }

    pub fn interpret_all<W: Write>(
        &self,
        statements: &Statements,
        env: &mut Env,
        out: &mut W,
    ) -> Result<(), ValueError> {
        for statement in statements {
            self.interpret(statement, env, out)?;
        }

        Ok(())
    }
}
