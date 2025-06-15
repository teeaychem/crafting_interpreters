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

    Unexpected { found: TknK },

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
    pub fn stumble<S: Into<StumbleKind>>(&self, kind: S) -> Stumble {
        Stumble {
            location: self.location,
            kind: kind.into(),
        }
    }
}
