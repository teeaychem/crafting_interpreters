use core::panic;

use super::{Expression, Interpreter};
use crate::{
    ast::{expression::{BinaryOp, UnaryOp}, literal::Literal},
    parser::value::{Value, ValueError},
};

impl Interpreter<'_> {
    pub fn evaluate_boolean(&self, expr: &Expression) -> Result<bool, ValueError> {
        match self.evaluate(expr)?.to_boolean() {
            Ok(Value::Boolean { b }) => Ok(b),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn evaluate_numeric(&self, expr: &Expression) -> Result<f64, ValueError> {
        match self.evaluate(expr)?.to_numeric() {
            Ok(Value::Numeric { n }) => Ok(n),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn evaluate_string(&self, expr: &Expression) -> Result<String, ValueError> {
        match self.evaluate(expr)?.to_string() {
            Ok(Value::String { s }) => Ok(s.to_owned()),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    pub fn evaluate(&self, expr: &Expression) -> Result<Value, ValueError> {
        let value = match expr {
            Expression::Literal { l } => Value::from(l.to_owned()),

            Expression::Identifier { l } => {
                match l {
                    Literal::String { s } => {
                        match self.e.get(s) {
                            Some(e) => self.evaluate(e)?,
                        

                            None => panic!("uf")
                        }
                    },

                    _ => todo!("Eval declaration")
                }
            },

            Expression::Grouping { e } => self.evaluate(e)?,

            Expression::Unary { op, e } => {
                use UnaryOp::*;
                match op {
                    Minus => Value::from(-self.evaluate_numeric(e)?),

                    Bang => Value::from(!(self.evaluate_boolean(e)?)),
                }
            }

            Expression::Binary { op, l, r } => {
                use BinaryOp::*;
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
        let interpreter = Interpreter::new();
        let number = Expression::from(64.0);

        let number_negation = Expression::unary(UnaryOp::Minus, number);

        assert_eq!(
            interpreter.evaluate(&number_negation),
            Ok(Value::from(-64.0))
        );

        let string = Expression::from("64");

        let string_negation = Expression::unary(UnaryOp::Minus, string);

        assert_eq!(
            interpreter.evaluate(&string_negation),
            Ok(Value::from(-64.0))
        );
    }

    #[test]
    fn basic_arithmetic() {
        let interpreter = Interpreter::new();

        let a_value = 64.0;
        let b_value = 32.0;

        let a = Expression::from(a_value);
        let b = Expression::from(b_value);

        let addition = Expression::binary(BinaryOp::Star, a, b);

        assert_eq!(
            interpreter.evaluate(&addition),
            Ok(Value::from(a_value * b_value))
        );
    }

    #[test]
    fn basic_string() {
        let interpreter = Interpreter::new();

        let a = Expression::from("a ");
        let b = Expression::from("string");

        let addition = Expression::binary(BinaryOp::Plus, a, b);

        assert_eq!(interpreter.evaluate(&addition), Ok(Value::from("a string")));
    }

    #[test]
    fn basic_comparison() {
        let interpreter = Interpreter::new();

        let a_value = 64.0;
        let b_value = 32.0;

        let gt = Expression::binary(
            BinaryOp::Gt,
            Expression::from(a_value),
            Expression::from(b_value),
        );
        let leq = Expression::binary(
            BinaryOp::Leq,
            Expression::from(a_value),
            Expression::from(b_value),
        );

        assert_eq!(interpreter.evaluate(&gt), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&leq), Ok(Value::from(false)));
    }

    #[test]
    fn basic_equality() {
        let interpreter = Interpreter::new();

        
        let a_value = 64.0;
        let b_value = 32.0;

        let eq_self = Expression::binary(
            BinaryOp::Eq,
            Expression::from(a_value),
            Expression::from(a_value),
        );

        let eq_same = Expression::binary(
            BinaryOp::Eq,
            Expression::from(Expression::from("a")),
            Expression::from(Expression::from("a")),
        );

        let neq = Expression::binary(
            BinaryOp::Eq,
            Expression::from(a_value),
            Expression::from(b_value),
        );

        let neq_different_types = Expression::binary(
            BinaryOp::Eq,
            Expression::from("64.0"),
            Expression::from(64.0),
        );

        assert_eq!(interpreter.evaluate(&eq_self), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&eq_same), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&neq), Ok(Value::from(false)));
        assert_eq!(interpreter.evaluate(&neq_different_types), Ok(Value::from(false)));
    }
}
