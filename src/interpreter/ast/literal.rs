use crate::interpreter::evaluation::value::Assignment;
use std::{fmt::Display, panic};

use crate::interpreter::ast::expression::Expression;

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

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal::String {
            s: value.to_owned(),
        }
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::String { s: value }
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

impl From<Literal> for Assignment {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Nil => Assignment::Nil,

            Literal::False => Assignment::from(false),

            Literal::True => Assignment::from(true),

            Literal::Number { n } => Assignment::from(n),

            Literal::String { s } => Assignment::from(s.to_owned()),
        }
    }
}
