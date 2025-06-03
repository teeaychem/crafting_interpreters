use crate::interpreter::location::Location;

pub type Tokens = Vec<Token>;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
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

impl Token {
    pub fn new(instance: TokenKind, location: Location) -> Self {
        Token {
            kind: instance,
            location,
        }
    }

    pub fn is(&self, instance: TokenKind) -> bool {
        self.kind == instance
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenKind::Number { literal } => write!(f, "Number: {literal}"),
            TokenKind::String { literal } => write!(f, "String: {literal}",),
            _ => write!(f, "Non-literal: {:?}", self.kind),
        }
    }
}

pub enum TokenError {
    TrailingDot,
    MultilineString,
    Unrecognised { character: char },
}
