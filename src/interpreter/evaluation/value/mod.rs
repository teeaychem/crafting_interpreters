mod conversion;

use crate::interpreter::environment::EnvErr;

#[derive(Debug, PartialEq)]
pub enum EvalErr {
    ConflictingSubexpression,
    InvalidConversion,
    InvalidAsignTo,
    InvalidIdentifier { id: String },
    EnvErr { err: EnvErr },
}
