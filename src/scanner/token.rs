use crate::location::Location;

pub type Tokens = Vec<Token>;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub instance: TokenInstance,
    pub location: Location,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum TokenInstance {
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

    // literals
    Identifier { literal: String },
    Number { literal: f64 },
    String { literal: String },

    // keywords
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

    //
    EOF,
}

impl Token {
    pub fn new(instance: TokenInstance, location: Location) -> Self {
        Token { instance, location }
    }

    pub fn is(&self, instance: TokenInstance) -> bool {
        self.instance == instance
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.instance {
            TokenInstance::Number { literal } => write!(f, "Number: {literal}"),
            TokenInstance::String { literal } => write!(f, "String: {literal}",),
            _ => write!(f, "Non-literal: {:?}", self.instance),
        }
    }
}

pub enum TokenError {
    TrailingDot,
    MultilineString,
    Unrecognised { character: char },
}
