use std::{fmt::Display, panic};

use crate::interpreter::{ast::expression::Expression, parser::value::Value};

#[derive(Clone, Debug)]
pub enum Literal {
    Number { n: f64 },
    String { s: String },
    True,
    False,
    Nil,
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Number { n: value }
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        Expression::from(Literal::from(value))
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal::String {
            s: value.to_owned(),
        }
    }
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        Expression::from(Literal::from(value))
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::String { s: value }
    }
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Expression::from(Literal::from(value))
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number { n } => write!(f, "{n}"),
            Self::String { s } => write!(f, "{s}"),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl From<Literal> for Value {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Nil => Value::Nil,

            Literal::False => Value::from(false),

            Literal::True => Value::from(true),

            Literal::Number { n } => Value::from(n),

            Literal::String { s } => Value::from(s.to_owned()),
        }
    }
}
