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

    ExpectedFound { expected: TknK, found: TknK },

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

    pub fn stumble_token_index<S: Into<StumbleKind>>(&self, kind: S) -> Stumble {
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
}
