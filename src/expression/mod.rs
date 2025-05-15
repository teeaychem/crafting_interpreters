use std::fmt::Display;

mod literal;
pub use literal::Literal;

#[derive(Debug)]
pub enum Expression {
    Literal {
        l: Literal,
    },

    Unary {
        op: UnaryOp,
        e: Box<Expression>,
    },

    Binary {
        op: BinaryOp,
        l: Box<Expression>,
        r: Box<Expression>,
    },

    Grouping {
        e: Box<Expression>,
    },
}



#[derive(Debug)]
pub enum UnaryOp {
    Minus,
    Bang,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),

            Self::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug)]
pub enum BinaryOp {
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

impl Display for BinaryOp {
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
            Self::Literal { l } => write!(f, "{l}"),
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
            op: BinaryOp::Star,
            l: Box::new(Expression::Unary {
                op: UnaryOp::Minus,
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
