use crate::interpreter::ast::{
    expression::{Expression, OpOne, OpTwo},
    literal::Literal,
};

use super::Basic;

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

    pub fn mk_numeric(n: f64) -> Self {
        Expression::Basic(Basic::Number { n })
    }

    pub fn mk_string(s: String) -> Self {
        Expression::Basic(Basic::String { s })
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
        Expression::Basic(Basic::True)
    }

    pub fn mk_false() -> Self {
        Expression::Basic(Basic::False)
    }

    pub fn mk_nil() -> Self {
        Expression::Basic(Basic::Nil)
    }

    pub fn mk_call(callee: Expression, args: Vec<Expression>) -> Self {
        Expression::Call {
            callee: Box::new(callee),
            args,
        }
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        Expression::Basic(Basic::Number { n: value })
    }
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        Expression::Basic(Basic::String {
            s: value.to_owned(),
        })
    }
}
