use crate::interpreter::ast::identifier::Identifier;
use crate::interpreter::evaluation::value::Value;
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
            Ok(Value::Boolean { b }) => Ok(b),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn eval_numeric(&self, expr: &Expression, env: &EnvHandle) -> Result<f64, EvalErr> {
        match self.eval(expr, env)?.to_numeric() {
            Ok(Value::Numeric { n }) => Ok(n),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn eval_string(&self, expr: &Expression, env: &EnvHandle) -> Result<String, EvalErr> {
        match self.eval(expr, env)?.to_string() {
            Ok(Value::String { s }) => Ok(s.to_owned()),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn get_identifier(&self, expr: &Expression) -> Result<Identifier, EvalErr> {
        match expr {
            Expression::Identifier { id: i } => Ok(i.to_owned()),

            _ => Err(EvalErr::InvalidAsignTo),
        }
    }

    pub fn eval(&self, expr: &Expression, env: &EnvHandle) -> Result<Value, EvalErr> {
        let value = match expr {
            Expression::Empty => Value::Nil,

            Expression::Literal { l } => Value::from(l.to_owned()),

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
                    Minus => Value::from(-self.eval_numeric(e, env)?),

                    Bang => Value::from(!(self.eval_boolean(e, env)?)),
                }
            }

            Expression::Binary { op, a: l, b: r } => {
                use OpTwo::*;
                match op {
                    Minus => Value::from(self.eval_numeric(l, env)? - self.eval_numeric(r, env)?),

                    Slash => Value::from(self.eval_numeric(l, env)? / self.eval_numeric(r, env)?),

                    Star => Value::from(self.eval_numeric(l, env)? * self.eval_numeric(r, env)?),

                    Plus => match (self.eval(l, env)?, self.eval(r, env)?) {
                        (Value::Numeric { n: l }, Value::Numeric { n: r }) => Value::from(l + r),

                        (Value::String { s: mut l }, Value::String { s: r }) => {
                            l.push_str(r.as_str());
                            Value::from(l)
                        }

                        _ => return Err(EvalErr::ConflictingSubexpression),
                    },

                    Gt => Value::from(self.eval_numeric(l, env)? > self.eval_numeric(r, env)?),

                    Geq => Value::from(self.eval_numeric(l, env)? >= self.eval_numeric(r, env)?),

                    Lt => Value::from(self.eval_numeric(l, env)? < self.eval_numeric(r, env)?),

                    Leq => Value::from(self.eval_numeric(l, env)? <= self.eval_numeric(r, env)?),

                    Eq => Value::from(self.eval(l, env)? == self.eval(r, env)?),

                    Neq => Value::from(self.eval(l, env)? != self.eval(r, env)?),
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
