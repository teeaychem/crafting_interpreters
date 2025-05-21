use crate::{ast::expression::Expression, parser::value::ValueError};

pub type Statements = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Expression {
        e: Expression,
    },

    Print {
        e: Expression,
    },

    Declaration {
        id: Expression,
        assignment: Expression,
    },

    Assignment {
        id: Expression,
        assignment: Expression,
    }
}
