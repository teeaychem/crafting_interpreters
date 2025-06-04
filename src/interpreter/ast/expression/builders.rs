use crate::interpreter::ast::{
    expression::{Expression, OpOne, OpTwo},
    literal::Literal,
};

impl Expression {
    pub fn mk_assignment(id: Expression, to: Expression) -> Self {
        Expression::Assignment {
            id: Box::new(id),
            e: Box::new(to),
        }
    }

    pub fn mk_binary(op: OpTwo, a: Expression, b: Expression) -> Self {
        Expression::Binary {
            op,
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn mk_unary(op: OpOne, a: Expression) -> Self {
        Expression::Unary { op, e: Box::new(a) }
    }

    pub fn mk_literal(literal: Literal) -> Self {
        Expression::Literal { l: literal }
    }

    pub fn mk_identifier(id: String) -> Self {
        Expression::Identifier { id }
    }

    pub fn mk_or(a: Expression, b: Expression) -> Self {
        Expression::Or {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn mk_and(a: Expression, b: Expression) -> Self {
        Expression::And {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn mk_true() -> Self {
        Expression::Literal { l: Literal::True }
    }

    pub fn mk_false() -> Self {
        Expression::Literal { l: Literal::True }
    }
}

impl From<Literal> for Expression {
    fn from(value: Literal) -> Self {
        Expression::Literal { l: value }
    }
}
