mod conversion;

use std::borrow::Borrow;

use crate::interpreter::environment::EnvErr;

#[derive(Clone, Debug, PartialEq)]
pub enum Assignment {
    Numeric { n: f64 },

    String { s: String },

    Boolean { b: bool },

    Nil,
}

#[derive(Debug, PartialEq)]
pub enum EvalErr {
    ConflictingSubexpression,
    InvalidConversion,
    InvalidAsignTo,
    InvalidIdentifier { id: String },
    EnvErr { err: EnvErr },
}

impl Assignment {
    pub fn is_truthy(&self) -> bool {
        match self {
            Assignment::Numeric { n } => true,
            Assignment::String { s } => true,
            Assignment::Boolean { b } => *b,
            Assignment::Nil => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        !self.is_truthy()
    }
}

impl From<&str> for Assignment {
    fn from(value: &str) -> Self {
        Assignment::String {
            s: value.to_owned(),
        }
    }
}

impl From<String> for Assignment {
    fn from(value: String) -> Self {
        Assignment::String { s: value }
    }
}

impl From<f64> for Assignment {
    fn from(value: f64) -> Self {
        Self::Numeric { n: value }
    }
}

impl From<bool> for Assignment {
    fn from(value: bool) -> Self {
        Self::Boolean { b: value }
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assignment::Nil => write!(f, "nil"),

            Assignment::Boolean { b } => write!(f, "{b}"),

            Assignment::String { s } => write!(f, "{s}"),

            Assignment::Numeric { n } => write!(f, "{n}"),
        }
    }
}
