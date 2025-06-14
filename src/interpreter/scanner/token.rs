use crate::interpreter::location::Location;

pub type Tkns = Vec<Tkn>;

#[derive(Clone, Debug, PartialEq)]
pub struct Tkn {
    pub kind: TknK,
    pub location: Location,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Debug, PartialEq)]
pub enum TknK {
    // Single character
    BraceL,
    BraceR,
    Comma,
    Dot,
    Minus,
    ParenL,
    ParenR,
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
    Break,
    Class,
    Else,
    False,
    For,
    Function,
    If,
    Loop,
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
    pub fn new(token: TknK, location: Location) -> Self {
        Tkn {
            kind: token,
            location,
        }
    }

    pub fn is(&self, instance: TknK) -> bool {
        self.kind == instance
    }
}

impl std::fmt::Display for Tkn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TknK::Number { literal } => write!(f, "Number: {literal}"),
            TknK::String { literal } => write!(f, "String: {literal}",),
            _ => write!(f, "Non-literal: {:?}", self.kind),
        }
    }
}

#[derive(Debug)]
pub enum TknErr {
    TrailingDot,
    MultilineString,
    Unrecognised { character: char },
}
