use core::panic;

use super::{Expression, Interpreter};
use crate::interpreter::{
    ast::{
        expression::{OpB, OpU},
        literal::{self, Literal},
    },
    parser::value::{Value, ValueError},
};

impl Interpreter<'_> {
    pub fn evaluate_boolean(&mut self, expr: &Expression) -> Result<bool, ValueError> {
        match self.evaluate(expr)?.to_boolean() {
            Ok(Value::Boolean { b }) => Ok(b),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn evaluate_numeric(&mut self, expr: &Expression) -> Result<f64, ValueError> {
        match self.evaluate(expr)?.to_numeric() {
            Ok(Value::Numeric { n }) => Ok(n),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn evaluate_string(&mut self, expr: &Expression) -> Result<String, ValueError> {
        match self.evaluate(expr)?.to_string() {
            Ok(Value::String { s }) => Ok(s.to_owned()),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn get_identifier(&mut self, expr: &Expression) -> Result<String, ValueError> {
        match expr {
            Expression::Identifier { id: i } => Ok(i.to_owned()),

            _ => Err(ValueError::InvalidAsignTo),
        }
    }

    pub fn evaluate(&mut self, expr: &Expression) -> Result<Value, ValueError> {
        let value = match expr {
            Expression::Empty => Value::Nil,

            Expression::Literal { l } => Value::from(l.to_owned()),

            Expression::Identifier { id } => match self.env.get(id) {
                None => return Err(ValueError::InvalidIdentifier),

                Some(e) => return Ok(e.to_owned()),
            },

            Expression::Assignment {
                id: name,
                e: assignment,
            } => {
                let assignment = self.evaluate(assignment)?;

                let name = self.get_identifier(name)?;

                self.env.assign(&name, assignment.clone());

                assignment
            }

            Expression::Grouping { e } => self.evaluate(e)?,

            Expression::Unary { op, e } => {
                use OpU::*;
                match op {
                    Minus => Value::from(-self.evaluate_numeric(e)?),

                    Bang => Value::from(!(self.evaluate_boolean(e)?)),
                }
            }

            Expression::Binary { op, l, r } => {
                use OpB::*;
                match op {
                    Minus => Value::from(self.evaluate_numeric(l)? - self.evaluate_numeric(r)?),

                    Slash => Value::from(self.evaluate_numeric(l)? / self.evaluate_numeric(r)?),

                    Star => Value::from(self.evaluate_numeric(l)? * self.evaluate_numeric(r)?),

                    Plus => match (self.evaluate(l)?, self.evaluate(r)?) {
                        (Value::Numeric { n: l }, Value::Numeric { n: r }) => Value::from(l + r),

                        (Value::String { s: mut l }, Value::String { s: r }) => {
                            l.push_str(r.as_str());
                            Value::from(l)
                        }

                        _ => return Err(ValueError::ConflictingSubexpression),
                    },

                    Gt => Value::from(self.evaluate_numeric(l)? > self.evaluate_numeric(r)?),

                    Geq => Value::from(self.evaluate_numeric(l)? >= self.evaluate_numeric(r)?),

                    Lt => Value::from(self.evaluate_numeric(l)? < self.evaluate_numeric(r)?),

                    Leq => Value::from(self.evaluate_numeric(l)? <= self.evaluate_numeric(r)?),

                    Eq => Value::from(self.evaluate(l)? == self.evaluate(r)?),

                    Neq => Value::from(self.evaluate(l)? != self.evaluate(r)?),
                }
            }

            Expression::Or { a, b } => {
                let a_value = self.evaluate(a)?;

                if a_value.is_truthy() {
                    a_value
                } else {
                    self.evaluate(b)?
                }
            }

            Expression::And { a, b } => {
                let a_value = self.evaluate(a)?;

                if a_value.is_falsey() {
                    a_value
                } else {
                    self.evaluate(b)?
                }
            }
        };

        Ok(value)
    }
}
