use super::{
    Expression, Interpreter,
    ast::identifier::Identifier,
    environment::{Env, EnvHandle},
};
use crate::interpreter::{
    ast::{
        expression::{OpOne, OpTwo},
        literal::{self, Literal},
    },
    parser::value::{Value, ValueError},
};

impl Interpreter {
    pub fn eval_boolean(&self, expr: &Expression, env: &EnvHandle) -> Result<bool, ValueError> {
        match self.evaluate(expr, env)?.to_boolean() {
            Ok(Value::Boolean { b }) => Ok(b),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn eval_numeric(&self, expr: &Expression, env: &EnvHandle) -> Result<f64, ValueError> {
        match self.evaluate(expr, env)?.to_numeric() {
            Ok(Value::Numeric { n }) => Ok(n),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn eval_string(&self, expr: &Expression, env: &EnvHandle) -> Result<String, ValueError> {
        match self.evaluate(expr, env)?.to_string() {
            Ok(Value::String { s }) => Ok(s.to_owned()),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn get_identifier(&self, expr: &Expression) -> Result<Identifier, ValueError> {
        match expr {
            Expression::Identifier { id: i } => Ok(i.to_owned()),

            _ => Err(ValueError::InvalidAsignTo),
        }
    }

    pub fn evaluate(&self, expr: &Expression, env: &EnvHandle) -> Result<Value, ValueError> {
        let value = match expr {
            Expression::Empty => Value::Nil,

            Expression::Literal { l } => Value::from(l.to_owned()),

            Expression::Identifier { id } => match env.borrow().get(id) {
                None => {
                    println!("{:?}", env);
                    return Err(ValueError::InvalidIdentifier { id: id.to_owned() });
                }

                Some(e) => return Ok(e.to_owned()),
            },

            Expression::Assignment {
                id: name,
                e: assignment,
            } => {
                let assignment = self.evaluate(assignment, env)?;

                let name = self.get_identifier(name)?;

                match env.borrow_mut().assign(&name, assignment.clone()) {
                    Ok(_) => {}

                    Err(e) => return Err(ValueError::EnvErr { err: e }),
                };

                assignment
            }

            Expression::Grouping { e } => self.evaluate(e, env)?,

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

                    Plus => match (self.evaluate(l, env)?, self.evaluate(r, env)?) {
                        (Value::Numeric { n: l }, Value::Numeric { n: r }) => Value::from(l + r),

                        (Value::String { s: mut l }, Value::String { s: r }) => {
                            l.push_str(r.as_str());
                            Value::from(l)
                        }

                        _ => return Err(ValueError::ConflictingSubexpression),
                    },

                    Gt => Value::from(self.eval_numeric(l, env)? > self.eval_numeric(r, env)?),

                    Geq => Value::from(self.eval_numeric(l, env)? >= self.eval_numeric(r, env)?),

                    Lt => Value::from(self.eval_numeric(l, env)? < self.eval_numeric(r, env)?),

                    Leq => Value::from(self.eval_numeric(l, env)? <= self.eval_numeric(r, env)?),

                    Eq => Value::from(self.evaluate(l, env)? == self.evaluate(r, env)?),

                    Neq => Value::from(self.evaluate(l, env)? != self.evaluate(r, env)?),
                }
            }

            Expression::Or { a, b } => {
                let a_value = self.evaluate(a, env)?;

                if a_value.is_truthy() {
                    a_value
                } else {
                    self.evaluate(b, env)?
                }
            }

            Expression::And { a, b } => {
                let a_value = self.evaluate(a, env)?;

                if a_value.is_falsey() {
                    a_value
                } else {
                    self.evaluate(b, env)?
                }
            }

            Expression::Call { callee, args } => {
                todo!("{expr}")
            }
        };

        Ok(value)
    }
}
