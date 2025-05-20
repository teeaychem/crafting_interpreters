use core::panic;

use super::{BinaryOp, Expression, Literal, UnaryOp};
use crate::parser::{
    evaluate::Evaluate,
    value::{Value, ValueError},
};

impl Evaluate for Expression {
    fn evaluate_boolean(&self) -> Result<bool, ValueError> {
        match self.evaluate()?.to_boolean() {
            Ok(Value::Boolean { b }) => Ok(b),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    fn evaluate_numeric(&self) -> Result<f64, ValueError> {
        match self.evaluate()?.to_numeric() {
            Ok(Value::Numeric { n }) => Ok(n),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    fn evaluate_string(&self) -> Result<String, ValueError> {
        match self.evaluate()?.to_string() {
            Ok(Value::String { s }) => Ok(s.to_owned()),

            _ => Err(ValueError::ConflictingSubexpression),
        }
    }

    fn evaluate(&self) -> Result<Value, ValueError> {
        let value = match self {
            Expression::Literal { l } => Value::from(l.to_owned()),

            Expression::Grouping { e } => e.evaluate()?,

            Expression::Unary { op, e } => {
                use UnaryOp::*;
                match op {
                    Minus => Value::from(-e.evaluate_numeric()?),

                    Bang => Value::from(!(e.evaluate_boolean()?)),
                }
            }

            Expression::Binary { op, l, r } => {
                use BinaryOp::*;
                match op {
                    Minus => Value::from(l.evaluate_numeric()? - r.evaluate_numeric()?),

                    Slash => Value::from(l.evaluate_numeric()? / r.evaluate_numeric()?),

                    Star => Value::from(l.evaluate_numeric()? * r.evaluate_numeric()?),

                    Plus => match (l.evaluate()?, r.evaluate()?) {
                        (Value::Numeric { n: l }, Value::Numeric { n: r }) => Value::from(l + r),

                        (Value::String { s: mut l }, Value::String { s: r }) => {
                            l.push_str(r.as_str());
                            Value::from(l)
                        }

                        _ => return Err(ValueError::ConflictingSubexpression),
                    },

                    Gt => Value::from(l.evaluate_numeric()? > r.evaluate_numeric()?),

                    Geq => Value::from(l.evaluate_numeric()? >= r.evaluate_numeric()?),

                    Lt => Value::from(l.evaluate_numeric()? < r.evaluate_numeric()?),

                    Leq => Value::from(l.evaluate_numeric()? <= r.evaluate_numeric()?),

                    Eq => Value::from(l.evaluate()? == r.evaluate()?),

                    Neq => Value::from(l.evaluate()? != r.evaluate()?),
                }
            }
        };

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_negation() {
        let number = Expression::from(64.0);

        let number_negation = Expression::unary(UnaryOp::Minus, number);

        assert_eq!(number_negation.evaluate(), Ok(Value::from(-64.0)));

        let string = Expression::from("64");

        let string_negation = Expression::unary(UnaryOp::Minus, string);

        assert_eq!(string_negation.evaluate(), Ok(Value::from(-64.0)));
    }

    #[test]
    fn basic_arithmetic() {
        let a_value = 64.0;
        let b_value = 32.0;

        let a = Expression::from(a_value);
        let b = Expression::from(b_value);

        let addition = Expression::binary(BinaryOp::Star, a, b);

        assert_eq!(addition.evaluate(), Ok(Value::from(a_value * b_value)));
    }

    #[test]
    fn basic_string() {
        let a = Expression::from("a ");
        let b = Expression::from("string");

        let addition = Expression::binary(BinaryOp::Plus, a, b);

        assert_eq!(addition.evaluate(), Ok(Value::from("a string")));
    }

    #[test]
    fn basic_comparison() {
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

        assert_eq!(gt.evaluate(), Ok(Value::from(true)));
        assert_eq!(leq.evaluate(), Ok(Value::from(false)));
    }

    #[test]
    fn basic_equality() {
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

        assert_eq!(eq_self.evaluate(), Ok(Value::from(true)));
        assert_eq!(eq_same.evaluate(), Ok(Value::from(true)));
        assert_eq!(neq.evaluate(), Ok(Value::from(false)));
        assert_eq!(neq_different_types.evaluate(), Ok(Value::from(false)));
    }
}
