use crate::interpreter::{
    Statement,
    ast::{
        expression::{Expr, ExprB},
        identifier::Identifier,
    },
};

use super::Statements;

impl Statement {
    pub fn mk_block(statements: Vec<Statement>) -> Self {
        Self::Block { statements }
    }

    pub fn mk_expression(e: Expr) -> Self {
        Self::Expression { e }
    }

    pub fn mk_print(e: Expr) -> Self {
        Self::Print { e }
    }

    pub fn mk_declaration(id: Identifier, e: Expr) -> Self {
        Statement::Declaration { id, e }
    }

    pub fn mk_assignment(id: Expr, e: Expr) -> Self {
        Statement::Assignment { id, e }
    }

    pub fn mk_conditional(
        condition: Expr,
        case_if: Statement,
        case_else: Option<Statement>,
    ) -> Self {
        Statement::Conditional {
            condition,
            case_if: Box::new(case_if),
            case_else: case_else.map(Box::new),
        }
    }

    pub fn mk_loop(statements: Vec<Statement>) -> Self {
        Statement::Loop { statements }
    }

    pub fn mk_while(condition: Expr, body: Statements) -> Self {
        Statement::While { condition, body }
    }

    pub fn mk_function(head: Identifier, args: Vec<Identifier>, body: Statements) -> Self {
        Statement::Function {
            id: head,
            parameters: args,
            body,
        }
    }
}
