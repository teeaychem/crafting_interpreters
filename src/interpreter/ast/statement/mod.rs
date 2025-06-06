use crate::interpreter::ast::{expression::Expr, identifier::Identifier};

mod builders;

pub type Statements = Vec<Statement>;

#[derive(Clone, Debug)]
pub enum Statement {
    Empty,

    Expression {
        e: Expr,
    },

    Print {
        e: Expr,
    },

    Declaration {
        id: Expr,
        e: Expr,
    },

    Assignment {
        id: Expr,
        e: Expr,
    },

    Conditional {
        condition: Expr,
        case_if: Box<Statement>,
        case_else: Option<Box<Statement>>,
    },

    While {
        condition: Expr,
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
