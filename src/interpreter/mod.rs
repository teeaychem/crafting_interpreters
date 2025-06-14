use std::io::Write;

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

pub enum Control {
    Break,
    Proceed,
}

impl Interpreter {
    pub fn interpret(
        &self,
        statement: &Statement,
        env: &EnvHandle,
        base: &mut Base,
    ) -> Result<(Control, ExprB), EvalErr> {
        match statement {
            Statement::Expression { e } => Ok((Control::Proceed, self.eval(e, env, base)?)),

            Statement::Print { e } => {
                let evaluation = self.eval(e, env, base)?;

                let _ = base.stdio.write(format!("{evaluation}\n").as_bytes());

                Ok((Control::Proceed, ExprB::Nil))
            }

            Statement::Declaration { id, e } => {
                let assignment = self.eval(e, env, base)?;

                env.borrow_mut().insert(id.name(), assignment.clone());

                Ok((Control::Proceed, assignment))
            }

            Statement::Block { statements } => {
                let block_env = Env::narrow(env.clone());

                let mut block_return = ExprB::Nil;
                let mut block_control = Control::Proceed;

                for statement in statements {
                    (block_control, block_return) = self.interpret(statement, &block_env, base)?;
                }

                Ok((block_control, block_return))
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
                    Ok((Control::Proceed, ExprB::Nil))
                }
            }

            Statement::Loop { statements } => {
                let block_env = Env::narrow(env.clone());

                let mut block_ctl;
                let mut block_rtn;

                'loop_loop: loop {
                    for statement in statements {
                        (block_ctl, block_rtn) = self.interpret(statement, &block_env, base)?;

                        match block_ctl {
                            Control::Break => break 'loop_loop,

                            Control::Proceed => {}
                        };
                    }
                }

                Ok((block_ctl, block_rtn))
            }

            Statement::While { condition, body } => {
                // TODO: Avoid a fresh block each time?

                let mut loops = vec![];

                loops.push(Statement::mk_conditional(
                    condition.clone(),
                    Statement::Empty,
                    Some(Statement::Break),
                ));
                loops.extend_from_slice(body);

                self.interpret(&Statement::mk_loop(loops), env, base)
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

                Ok((Control::Proceed, ExprB::Nil))
            }

            Statement::Return { expr } => Ok((Control::Proceed, self.eval(expr, env, base)?)),

            Statement::Break => Ok((Control::Break, ExprB::Nil)),

            Statement::Empty => Ok((Control::Proceed, ExprB::Nil)),

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
