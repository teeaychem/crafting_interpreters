use crate::interpreter::location::Location;

pub type Tkns = Vec<Tkn>;

#[derive(Clone, Debug, PartialEq)]
pub struct Tkn {
    pub kind: TknKind,
    pub location: Location,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum TknKind {
    // Single character
    BraceLeft,
    BraceRight,
    Comma,
    Dot,
    Minus,
    ParenLeft,
    ParenRight,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier { id: String },
    Number { literal: f64 },
    String { literal: String },

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Invisible
    EOF,
}

impl Tkn {
    pub fn new(instance: TknKind, location: Location) -> Self {
        Tkn {
            kind: instance,
            location,
        }
    }

    pub fn is(&self, instance: TknKind) -> bool {
        self.kind == instance
    }
}

impl std::fmt::Display for Tkn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TknKind::Number { literal } => write!(f, "Number: {literal}"),
            TknKind::String { literal } => write!(f, "String: {literal}",),
            _ => write!(f, "Non-literal: {:?}", self.kind),
        }
    }
}

pub enum TknErr {
    TrailingDot,
    MultilineString,
    Unrecognised { character: char },
}
