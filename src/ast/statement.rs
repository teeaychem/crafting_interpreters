use crate::{
    ast::expression::Expression,
    parser::{evaluate::Evaluate, value::ValueError},
};

pub type Statements = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Expression { e: Expression },
    Print { e: Expression },
}

