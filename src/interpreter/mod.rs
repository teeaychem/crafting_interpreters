use std::{
    collections::HashMap,
    io::{BufRead, Write},
};

pub mod ast;
pub mod location;

pub mod environment;
pub mod evaluation;

mod parser;
pub use parser::Parser;

mod scanner;
pub use scanner::Scanner;

mod base;
pub use base::Base;

use ast::{
    expression::ExprB,
    statement::{Statement, Statements},
};
use environment::{Env, EnvHandle};
use evaluation::value::EvalErr;

#[cfg(test)]
mod tests;

pub struct Interpreter {}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {}
    }
}

impl Interpreter {
    pub fn interpret(
        &self,
        statement: &Statement,
        env: &EnvHandle,
        base: &mut Base,
    ) -> Result<Option<ExprB>, EvalErr> {
        let mut return_expr = None;

        match statement {
            Statement::Expression { e } => {
                self.eval(e, env, base)?;
            }

            Statement::Print { e } => {
                let evaluation = self.eval(e, env, base)?;

                unsafe {
                    base.stdio.write(format!("{evaluation}\n").as_bytes());
                }
            }

            Statement::Declaration { id, e } => {
                let assignment = self.eval(e, env, base)?;

                return_expr = Some(assignment.clone());

                env.borrow_mut().insert(id.name(), assignment);
            }

            Statement::Block { statements } => {
                let mut nenv = Env::narrow(env.clone());

                for statement in statements {
                    return_expr = self.interpret(statement, &nenv, base)?;
                }
            }

            Statement::Conditional {
                condition,
                case_if: yes,
                case_else: no,
            } => {
                if self.eval(condition, env, base)?.is_truthy() {
                    return_expr = self.interpret(yes, env, base)?;
                } else if let Some(no) = no {
                    return_expr = self.interpret(no, env, base)?;
                }
            }

            Statement::While { condition, body } => {
                while self.eval(condition, env, base)?.is_truthy() {
                    self.interpret(body, env, base);
                }
            }

            Statement::Fun {
                id,
                parameters,
                body,
            } => {
                let lambda = ExprB::Lambda {
                    env: env.clone(),
                    params: parameters.clone(),
                    body: body.clone(),
                };

                env.borrow_mut().insert(id.name(), lambda);
            }

            Statement::Return { expr } => return_expr = Some(self.eval(expr, env, base)?),

            _ => todo!("Inpereter todo: {statement:?}"),
        }

        Ok(return_expr)
    }

    pub fn interpret_all(
        &self,
        statements: &Statements,
        env: &EnvHandle,
        base: &mut Base,
    ) -> Result<(), EvalErr> {
        for statement in statements {
            self.interpret(statement, env, base)?;
        }

        Ok(())
    }
}
