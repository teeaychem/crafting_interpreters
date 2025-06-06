use crate::interpreter::ast::expression::Expression;

use super::{expression, identifier::Identifier};

mod builders;

pub type Statements = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Empty,

    Expression {
        e: Expression,
    },

    Print {
        e: Expression,
    },

    Declaration {
        id: Expression,
        e: Expression,
    },

    Assignment {
        id: Expression,
        e: Expression,
    },

    Conditional {
        condition: Expression,
        case_if: Box<Statement>,
        case_else: Option<Box<Statement>>,
    },

    While {
        condition: Expression,
        body: Box<Statement>,
    },

    Block {
        statements: Vec<Statement>,
    },

    Fun {
        id: Identifier,
        parameters: Vec<Identifier>,
        body: Statements,
    },
}
