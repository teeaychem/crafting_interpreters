use crate::interpreter::ast::expression::Basic;
use crate::interpreter::ast::identifier::Identifier;
use crate::interpreter::evaluation::value::Assignment;
use crate::{
    Interpreter,
    interpreter::{
        ast::expression::{Expression, OpOne, OpTwo},
        environment::EnvHandle,
    },
};

use crate::interpreter::evaluation::value::EvalErr;

impl Interpreter {
    pub fn eval_boolean(&self, expr: &Expression, env: &EnvHandle) -> Result<bool, EvalErr> {
        match self.eval(expr, env)?.to_boolean() {
            Ok(Assignment::Boolean { b }) => Ok(b),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn eval_numeric(&self, expr: &Expression, env: &EnvHandle) -> Result<f64, EvalErr> {
        match self.eval(expr, env)?.to_numeric() {
            Ok(Assignment::Numeric { n }) => Ok(n),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn eval_string(&self, expr: &Expression, env: &EnvHandle) -> Result<String, EvalErr> {
        match self.eval(expr, env)?.to_string() {
            Ok(Assignment::String { s }) => Ok(s.to_owned()),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn get_identifier(&self, expr: &Expression) -> Result<Identifier, EvalErr> {
        match expr {
            Expression::Identifier { id: i } => Ok(i.to_owned()),

            _ => Err(EvalErr::InvalidAsignTo),
        }
    }

    pub fn eval(&self, expr: &Expression, env: &EnvHandle) -> Result<Assignment, EvalErr> {
        let value = match expr {
            Expression::Empty => Assignment::Nil,

            Expression::Basic(bexpr) => {
                //
                match bexpr {
                    Basic::Nil => Assignment::Nil,

                    Basic::False => Assignment::Boolean { b: false },

                    Basic::True => Assignment::Boolean { b: true },

                    Basic::Number { n } => Assignment::Numeric { n: *n },

                    Basic::String { s } => Assignment::String { s: s.to_owned() },
                }
            }

            Expression::Identifier { id } => match env.borrow().get(id) {
                None => {
                    println!("{:?}", env);
                    return Err(EvalErr::InvalidIdentifier { id: id.to_owned() });
                }

                Some(e) => return Ok(e.to_owned()),
            },

            Expression::Assignment {
                id: name,
                e: assignment,
            } => {
                let assignment = self.eval(assignment, env)?;

                let name = self.get_identifier(name)?;

                match env.borrow_mut().assign(&name, assignment.clone()) {
                    Ok(_) => {}

                    Err(e) => return Err(EvalErr::EnvErr { err: e }),
                };

                assignment
            }

            Expression::Grouping { e } => self.eval(e, env)?,

            Expression::Unary { op, e } => {
                use OpOne::*;
                match op {
                    Minus => Assignment::from(-self.eval_numeric(e, env)?),

                    Bang => Assignment::from(!(self.eval_boolean(e, env)?)),
                }
            }

            Expression::Binary { op, a: l, b: r } => {
                use OpTwo::*;
                match op {
                    Minus => {
                        Assignment::from(self.eval_numeric(l, env)? - self.eval_numeric(r, env)?)
                    }

                    Slash => {
                        Assignment::from(self.eval_numeric(l, env)? / self.eval_numeric(r, env)?)
                    }

                    Star => {
                        Assignment::from(self.eval_numeric(l, env)? * self.eval_numeric(r, env)?)
                    }

                    Plus => match (self.eval(l, env)?, self.eval(r, env)?) {
                        (Assignment::Numeric { n: l }, Assignment::Numeric { n: r }) => {
                            Assignment::from(l + r)
                        }

                        (Assignment::String { s: mut l }, Assignment::String { s: r }) => {
                            l.push_str(r.as_str());
                            Assignment::from(l)
                        }

                        _ => return Err(EvalErr::ConflictingSubexpression),
                    },

                    Gt => Assignment::from(self.eval_numeric(l, env)? > self.eval_numeric(r, env)?),

                    Geq => {
                        Assignment::from(self.eval_numeric(l, env)? >= self.eval_numeric(r, env)?)
                    }

                    Lt => Assignment::from(self.eval_numeric(l, env)? < self.eval_numeric(r, env)?),

                    Leq => {
                        Assignment::from(self.eval_numeric(l, env)? <= self.eval_numeric(r, env)?)
                    }

                    Eq => Assignment::from(self.eval(l, env)? == self.eval(r, env)?),

                    Neq => Assignment::from(self.eval(l, env)? != self.eval(r, env)?),
                }
            }

            Expression::Or { a, b } => {
                let a_value = self.eval(a, env)?;

                if a_value.is_truthy() {
                    a_value
                } else {
                    self.eval(b, env)?
                }
            }

            Expression::And { a, b } => {
                let a_value = self.eval(a, env)?;

                if a_value.is_falsey() {
                    a_value
                } else {
                    self.eval(b, env)?
                }
            }

            Expression::Call { callee, args } => {
                todo!("{expr}")
            }
        };

        Ok(value)
    }
}
