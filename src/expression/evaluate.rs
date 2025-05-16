use core::panic;

use crate::value::{Evaluable, Value, ValueError};

use super::{BinaryOp, Expression, Literal, UnaryOp};

impl Evaluable for Expression {
    fn evaluate_boolean(&self) -> Result<bool, ValueError> {
        match self.evaluate()?.to_numeric() {
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

                    Bang => Value::from(!e.evaluate_boolean()?),
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

                    _ => todo!(),
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
}
