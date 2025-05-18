mod conversion;

use std::borrow::Borrow;

#[derive(Clone, Debug)]
pub enum Value {
    Numeric { n: f64 },
    String { s: String },
    Boolean { b: bool },
    Null,
}

#[derive(Debug, PartialEq)]
pub enum ValueError {
    ConflictingSubexpression,
    InvalidConversion,
}

pub trait Evaluable {
    fn evaluate(&self) -> Result<Value, ValueError>;

    fn evaluate_boolean(&self) -> Result<bool, ValueError>;
    fn evaluate_numeric(&self) -> Result<f64, ValueError>;
    fn evaluate_string(&self) -> Result<String, ValueError>;
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Boolean { .. }, Boolean { .. }) => true,

            (Numeric { .. }, Numeric { .. }) => true,

            (String { .. }, String { .. }) => true,

            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        use Value::*;

        match (self, other) {
            (Null, _) => None,
            (_, Null) => None,

            (Boolean { .. }, Boolean { .. }) => Some(Equal),
            (Boolean { .. }, _) => Some(Greater),

            (Numeric { .. }, Boolean { .. }) => Some(Less),
            (Numeric { .. }, Numeric { .. }) => Some(Equal),
            (Numeric { .. }, String { .. }) => Some(Greater),

            (String { .. }, String { .. }) => Some(Equal),
            (String { .. }, _) => Some(Less),
        }
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
            Value::Null => write!(f, "nil"),

            Value::Boolean { b } => write!(f, "{b}"),

            Value::String { s } => write!(f, "{s}"),

            Value::Numeric { n } => write!(f, "{n}"),
        }
    }
}
