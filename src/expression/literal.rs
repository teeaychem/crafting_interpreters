use std::fmt::Display;

#[derive(Debug)]
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
        Literal::String { s: value.to_owned() }
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
            Literal::True => write!(f, "True"),
            Literal::False => write!(f, "False"),
            Literal::Nil => write!(f, "Nil"),
        }
    }
}
