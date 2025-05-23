use crate::{ast::expression::Expression, parser::value::ValueError};

use super::expression;

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
        e: Expression,
    },

    Assignment {
        id: Expression,
        e: Expression,
    },

    Conditional {
        condition: Expression,
        yes: Box<Statement>,
        no: Option<Box<Statement>>,
    },

    BlockEnter,

    BlockExit,
}

impl Statement {
    pub fn expression(e: Expression) -> Self {
        Self::Expression { e }
    }

    pub fn print(e: Expression) -> Self {
        Self::Print { e }
    }

    pub fn declaration(id: Expression, e: Option<Expression>) -> Self {
        match e {
            Some(expr) => Statement::Declaration { id, e: expr },

            None => Statement::Declaration {
                id,
                e: Expression::Empty,
            },
        }
    }

    pub fn assignment(id: Expression, e: Expression) -> Self {
        Statement::Assignment { id, e }
    }
}
