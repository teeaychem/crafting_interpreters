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
        match statement {
            Statement::Expression { e } => Ok(Some(self.eval(e, env, base)?)),

            Statement::Print { e } => {
                let evaluation = self.eval(e, env, base)?;

                unsafe {
                    let _ = base.stdio.write(format!("{evaluation}\n").as_bytes());
                }

                Ok(Some(ExprB::Nil))
            }

            Statement::Declaration { id, e } => {
                let assignment = self.eval(e, env, base)?;

                env.borrow_mut().insert(id.name(), assignment.clone());

                Ok(Some(assignment))
            }

            Statement::Block { statements } => {
                let mut block_env = Env::narrow(env.clone());
                let mut block_return = Some(ExprB::Nil);

                for statement in statements {
                    block_return = self.interpret(statement, &block_env, base)?;
                }

                Ok(block_return)
            }

            Statement::Conditional {
                condition,
                case_if,
                case_else,
            } => {
                if self.eval(condition, env, base)?.is_truthy() {
                    Ok(self.interpret(case_if, env, base)?)
                } else if let Some(otherwise) = case_else {
                    Ok(self.interpret(otherwise, env, base)?)
                } else {
                    Ok(Some(ExprB::Nil))
                }
            }

            Statement::While { condition, body } => {
                // TODO: Avoid a fresh block each time?
                let mut block_return = Some(ExprB::Nil);

                while self.eval(condition, env, base)?.is_truthy() {
                    block_return = self.interpret(body, env, base)?;
                }

                Ok(block_return)
            }

            Statement::Function {
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

                Ok(Some(ExprB::Nil))
            }

            Statement::Return { expr } => Ok(Some(self.eval(expr, env, base)?)),

            Statement::Break => Ok(None),

            _ => todo!("Inpereter todo: {statement:?}"),
        }
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
