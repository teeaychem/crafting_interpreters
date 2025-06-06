use std::{collections::HashMap, io::Write};

use std::io::BufRead;

pub mod ast;
pub mod function;
pub mod location;
pub mod parser;
pub mod scanner;

use ast::expression::ExprB;
use environment::{Env, EnvHandle};
use evaluation::value::EvalErr;

use crate::interpreter::ast::statement::{Statement, Statements};

pub mod environment;
pub mod evaluation;

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
        env: &EnvHandle,
        out: &mut W,
    ) -> Result<(), EvalErr> {
        match statement {
            Statement::Expression { e } => {
                self.eval(e, env, out)?;
            }

            Statement::Print { e } => {
                let evaluation = self.eval(e, env, out)?;

                unsafe {
                    out.write(format!("{evaluation}\n").as_bytes());
                }
            }

            Statement::Declaration { id, e } => {
                let id = self.get_identifier(id)?;

                let assignment = self.eval(e, env, out)?;

                env.borrow_mut().insert(id, assignment);
            }

            Statement::Block { statements } => {
                let mut nenv = Env::narrow(env.clone());

                for statement in statements {
                    self.interpret(statement, &nenv, out);
                }
            }

            Statement::Conditional {
                condition,
                case_if: yes,
                case_else: no,
            } => {
                if self.eval(condition, env, out)?.is_truthy() {
                    self.interpret(yes, env, out);
                } else if let Some(no) = no {
                    self.interpret(no, env, out);
                }
            }

            Statement::While { condition, body } => {
                while self.eval(condition, env, out)?.is_truthy() {
                    self.interpret(body, env, out);
                }
            }

            Statement::Fun {
                id,
                parameters,
                body,
            } => {
                let lambda = ExprB::Lambda {
                    env: env.borrow().enclosing().expect("!"),
                    params: parameters.clone(),
                    body: body.clone(),
                };

                env.borrow_mut().insert(id.to_owned(), lambda);
            }

            _ => todo!("Inpereter todo: {statement:?}"),
        }

        Ok(())
    }

    pub fn interpret_all<W: Write>(
        &self,
        statements: &Statements,
        env: &EnvHandle,
        out: &mut W,
    ) -> Result<(), EvalErr> {
        for statement in statements {
            self.interpret(statement, env, out)?;
        }

        Ok(())
    }
}
