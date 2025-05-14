use std::fmt::Display;

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

pub enum Literal {
    Number { n: f64 },
    String { s: String },
    True,
    False,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number { n } => write!(f, "{n}"),
            Self::String { s } => write!(f, "{s}"),
            Literal::True => write!(f, "True"),
            Literal::False => write!(f, "False"),
            Literal::Nil => write!(f, "Nil"),
        }
    }
}

pub enum UnaryOp {
    Dash,
    Bang,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),

            Self::Dash => write!(f, "-"),
        }
    }
}

pub enum BinaryOp {
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geg,
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
            Self::Geg => write!(f, ">="),
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
                op: UnaryOp::Dash,
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
