use crate::interpreter::ast::{expression::Expr, identifier::Identifier};

use super::expression::ExprB;

mod builders;

pub type Statements = Vec<Statement>;

#[derive(Clone, Debug)]
pub enum Statement {
    Assignment {
        id: Expr,
        e: Expr,
    },

    Block {
        statements: Vec<Statement>,
    },

    Break,

    Conditional {
        condition: Expr,
        case_if: Box<Statement>,
        case_else: Option<Box<Statement>>,
    },

    Declaration {
        id: Identifier,
        e: Expr,
    },

    Empty,

    Expression {
        e: Expr,
    },

    Function {
        id: Identifier,
        parameters: Vec<Identifier>,
        body: Statements,
    },

    Print {
        e: Expr,
    },

    Return {
        expr: Expr,
    },

    While {
        condition: Expr,
        body: Statements,
    },

    Loop {
        statements: Vec<Statement>,
    },
}
