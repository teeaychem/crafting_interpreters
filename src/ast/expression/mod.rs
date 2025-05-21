use std::fmt::Display;

use super::literal::{self, Literal};

#[derive(Debug, Clone)]
pub enum Expression {
    Empty,

    Literal {
        l: Literal,
    },

    Identifier {
        id: Literal,
    },

    Assignment {
        id: Box<Expression>,
        e: Box<Expression>,
    },

    Unary {
        op: OpU,
        e: Box<Expression>,
    },

    Binary {
        op: OpB,
        l: Box<Expression>,
        r: Box<Expression>,
    },

    Grouping {
        e: Box<Expression>,
    },
}

impl Expression {
    pub fn assignment(id: Expression, to: Expression) -> Self {
        Expression::Assignment {
            id: Box::new(id),
            e: Box::new(to),
        }
    }

    pub fn binary(op: OpB, a: Expression, b: Expression) -> Self {
        Expression::Binary {
            op,
            l: Box::new(a),
            r: Box::new(b),
        }
    }

    pub fn unary(op: OpU, a: Expression) -> Self {
        Expression::Unary { op, e: Box::new(a) }
    }

    pub fn literal(literal: Literal) -> Self {
        Expression::Literal { l: literal }
    }

    pub fn identifier(id: Literal) -> Self {
        Expression::Identifier { id }
    }
}

impl From<Literal> for Expression {
    fn from(value: Literal) -> Self {
        Expression::Literal { l: value }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpU {
    Minus,
    Bang,
}

impl Display for OpU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),

            Self::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpB {
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,
    Plus,
    Minus,
    Star,
    Slash,
}

impl Display for OpB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "=="),
            Self::Neq => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Leq => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Geq => write!(f, ">="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "<Empty>"),
            Self::Literal { l } => write!(f, "{l}"),
            Self::Identifier { id: l } => write!(f, "{l}"),
            Expression::Assignment {
                id: name,
                e: assignment,
            } => write!(f, "{name} = {assignment}"),
            Self::Grouping { e } => write!(f, "(group {e})"),
            Self::Unary { op, e } => write!(f, "({op} {e})"),
            Self::Binary { op, l, r } => write!(f, "({op} {l} {r})"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_display() {
        let ast = Expression::Binary {
            op: OpB::Star,
            l: Box::new(Expression::Unary {
                op: OpU::Minus,
                e: Box::new(Expression::Literal {
                    l: Literal::Number { n: 123_f64 },
                }),
            }),
            r: Box::new(Expression::Grouping {
                e: Box::new(Expression::Literal {
                    l: Literal::Number { n: 45.67 },
                }),
            }),
        };

        assert_eq!(format!("{ast}"), "(* (- 123) (group 45.67))");
    }
}
