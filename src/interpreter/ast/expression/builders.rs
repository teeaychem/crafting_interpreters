use crate::interpreter::ast::expression::{Expr, OpOne, OpTwo};

use super::ExprB;

impl Expr {
    pub fn mk_assignment(id: Expr, to: Expr) -> Self {
        Expr::Assignment {
            id: Box::new(id),
            e: Box::new(to),
        }
    }

    pub fn mk_binary(op: OpTwo, a: Expr, b: Expr) -> Self {
        Expr::Binary {
            op,
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn mk_unary(op: OpOne, a: Expr) -> Self {
        Expr::Unary { op, e: Box::new(a) }
    }

    pub fn mk_numeric(n: f64) -> Self {
        Expr::Basic(ExprB::Numeric { n })
    }

    pub fn mk_string(s: String) -> Self {
        Expr::Basic(ExprB::String { s })
    }

    pub fn mk_identifier(id: String) -> Self {
        Expr::Identifier { id }
    }

    pub fn mk_or(a: Expr, b: Expr) -> Self {
        Expr::Or {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn mk_and(a: Expr, b: Expr) -> Self {
        Expr::And {
            a: Box::new(a),
            b: Box::new(b),
        }
    }

    pub fn mk_true() -> Self {
        Expr::Basic(ExprB::mk_bool(true))
    }

    pub fn mk_false() -> Self {
        Expr::Basic(ExprB::mk_bool(false))
    }

    pub fn mk_nil() -> Self {
        Expr::Basic(ExprB::Nil)
    }

    pub fn mk_call(caller: Expr, args: Vec<Expr>) -> Self {
        Expr::Call {
            caller: Box::new(caller),
            args,
        }
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Basic(ExprB::Numeric { n: value })
    }
}

impl From<&str> for Expr {
    fn from(value: &str) -> Self {
        Expr::Basic(ExprB::String {
            s: value.to_owned(),
        })
    }
}
