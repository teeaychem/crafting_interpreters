use crate::interpreter::{ast::expression::Expression, parser::value::ValueError};

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

    pub fn conditional(
        condition: Expression,
        case_if: Statement,
        case_else: Option<Statement>,
    ) -> Self {
        Statement::Conditional {
            condition,
            case_if: Box::new(case_if),
            case_else: match case_else {
                Some(case) => Some(Box::new(case)),
                None => None,
            },
        }
    }

    pub fn loop_while(condition: Expression, body: Statement) -> Self {
        Statement::While {
            condition,
            body: Box::new(body),
        }
    }
}
