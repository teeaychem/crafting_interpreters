use super::{
    ast::{identifier::Identifier, statement::Statements},
    environment::EnvHandle,
};

pub struct Function {
    arity: usize,
    args: Vec<Identifier>,
    body: Statements,
    env: EnvHandle,
}
