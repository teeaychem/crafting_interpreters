use super::{TreeWalker, environment::EnvErr, location::Location, scanner::token::TknK};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Stumble {
    pub location: Location,
    pub kind: StumbleKind,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum StumbleKind {
    // Parsing
    ArgLimit,

    ExpectedAssignment,

    ExpectedBlock,

    ExpectedFound { expected: TknK, found: TknK },

    ExpectedLambda,

    ForInitialiser,

    InvalidAsignee,

    MismatchedParentheses,

    MissingToken,

    OpenStatement,

    Todo,

    TokensExhausted,

    Unexpected(usize),

    // Evaluation
    ConflictingSubexpression,

    InvalidConversion,

    InvalidAssignTo,

    InvalidIdentifier { id: String },

    // Tokens
    MissingAsignee,

    TrailingDot,

    MultilineString,

    Unrecognised { character: char },
}

impl Stumble {
    pub fn kind(&self) -> &StumbleKind {
        &self.kind
    }
}

impl From<EnvErr> for StumbleKind {
    fn from(value: EnvErr) -> Self {
        match value {
            EnvErr::MissingAsignee => StumbleKind::MissingAsignee,
        }
    }
}

impl TreeWalker {
    pub fn stumble_here<S: Into<StumbleKind>>(&self, kind: S) -> Stumble {
        Stumble {
            location: self.parse_location,
            kind: kind.into(),
        }
    }

    pub fn stumble_token<S: Into<StumbleKind>>(&self, kind: S) -> Stumble {
        let kind = kind.into();

        let location: Location = match kind {
            StumbleKind::TokensExhausted => self.parse_location,

            StumbleKind::OpenStatement => match self.tokens.get(self.token_index - 1) {
                None => Location::default(),

                Some(t) => t.location,
            },

            _ => match self.tokens.get(self.token_index) {
                None => Location::default(),

                Some(t) => t.location,
            },
        };

        Stumble { location, kind }
    }

    pub fn handle_stumble(&self, stumble: &Stumble) {
        let line = stumble.location.line;
        let col = stumble.location.col;

        let src_start = self.line_breaks[line];
        let src_end = match self.line_breaks.get(line + 1) {
            Some(v) if *v != src_start => *v,
            _ => self.source.len() - 1,
        };

        let kind = &stumble.kind;

        match stumble.kind {
            StumbleKind::Unexpected(tkn) => {
                println!("{:?}", self.tokens[tkn])
            }

            _ => {}
        }

        let error_line: String = self.source[src_start..src_end].iter().collect();

        println!("Error on line {line} at column {col}: {kind:?}");
        println!("> {error_line}");
    }
}
