use core::panic;

use super::{Expression, Interpreter};
use crate::{
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
            Expression::Identifier {
                l: Literal::String { s },
            } => Ok(s.to_owned()),

            _ => {
                return Err(ValueError::InvalidAsignTo);
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Expression) -> Result<Value, ValueError> {
        let value = match expr {
            Expression::Literal { l } => Value::from(l.to_owned()),

            Expression::Identifier { l } => match l {
                Literal::String { s } => match self.env.get(s) {
                    None => return Err(ValueError::InvalidIdentifier),

                    Some(e) => return Ok(e.to_owned()),
                },

                _ => todo!("Eval declaration"),
            },

            Expression::Assignment { id: name, assignment } => {
                let assignment = self.evaluate(assignment)?;

                let name = self.get_identifier(name)?;

                self.env.insert(name, assignment.clone());

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
        };

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use crate::interpreter;

    use super::*;

    #[test]
    fn basic_negation() {
        let mut interpreter = Interpreter::new();
        let number = Expression::from(64.0);

        let number_negation = Expression::unary(OpU::Minus, number);

        assert_eq!(
            interpreter.evaluate(&number_negation),
            Ok(Value::from(-64.0))
        );

        let string = Expression::from("64");

        let string_negation = Expression::unary(OpU::Minus, string);

        assert_eq!(
            interpreter.evaluate(&string_negation),
            Ok(Value::from(-64.0))
        );
    }

    #[test]
    fn basic_arithmetic() {
        let mut interpreter = Interpreter::new();

        let a_value = 64.0;
        let b_value = 32.0;

        let a = Expression::from(a_value);
        let b = Expression::from(b_value);

        let addition = Expression::binary(OpB::Star, a, b);

        assert_eq!(
            interpreter.evaluate(&addition),
            Ok(Value::from(a_value * b_value))
        );
    }

    #[test]
    fn basic_string() {
        let mut interpreter = Interpreter::new();

        let a = Expression::from("a ");
        let b = Expression::from("string");

        let addition = Expression::binary(OpB::Plus, a, b);

        assert_eq!(interpreter.evaluate(&addition), Ok(Value::from("a string")));
    }

    #[test]
    fn basic_comparison() {
        let mut interpreter = Interpreter::new();

        let a_value = 64.0;
        let b_value = 32.0;

        let gt = Expression::binary(
            OpB::Gt,
            Expression::from(a_value),
            Expression::from(b_value),
        );
        let leq = Expression::binary(
            OpB::Leq,
            Expression::from(a_value),
            Expression::from(b_value),
        );

        assert_eq!(interpreter.evaluate(&gt), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&leq), Ok(Value::from(false)));
    }

    #[test]
    fn basic_equality() {
        let mut interpreter = Interpreter::new();

        let a_value = 64.0;
        let b_value = 32.0;

        let eq_self = Expression::binary(
            OpB::Eq,
            Expression::from(a_value),
            Expression::from(a_value),
        );

        let eq_same = Expression::binary(
            OpB::Eq,
            Expression::from(Expression::from("a")),
            Expression::from(Expression::from("a")),
        );

        let neq = Expression::binary(
            OpB::Eq,
            Expression::from(a_value),
            Expression::from(b_value),
        );

        let neq_different_types =
            Expression::binary(OpB::Eq, Expression::from("64.0"), Expression::from(64.0));

        assert_eq!(interpreter.evaluate(&eq_self), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&eq_same), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&neq), Ok(Value::from(false)));
        assert_eq!(
            interpreter.evaluate(&neq_different_types),
            Ok(Value::from(false))
        );
    }
}
