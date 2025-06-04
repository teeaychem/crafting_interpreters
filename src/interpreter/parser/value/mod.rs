mod conversion;

use std::borrow::Borrow;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Numeric { n: f64 },
    String { s: String },
    Boolean { b: bool },
    Nil,
}

#[derive(Debug, PartialEq)]
pub enum ValueError {
    ConflictingSubexpression,
    InvalidConversion,
    InvalidAsignTo,
    InvalidIdentifier { id: String },
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Numeric { n } => true,
            Value::String { s } => true,
            Value::Boolean { b } => *b,
            Value::Nil => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        !self.is_truthy()
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String {
            s: value.to_owned(),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String { s: value }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Numeric { n: value }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean { b: value }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),

            Value::Boolean { b } => write!(f, "{b}"),

            Value::String { s } => write!(f, "{s}"),

            Value::Numeric { n } => write!(f, "{n}"),
        }
    }
}
