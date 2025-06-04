mod builders;
mod operators;

pub use operators::{OpOne, OpTwo};

use super::literal::Literal;

#[derive(Debug, Clone)]
pub enum Expression {
    Empty,

    Literal {
        l: Literal,
    },

    Identifier {
        id: String,
    },

    Assignment {
        id: Box<Expression>,
        e: Box<Expression>,
    },

    Unary {
        op: OpOne,
        e: Box<Expression>,
    },

    Binary {
        op: OpTwo,
        a: Box<Expression>,
        b: Box<Expression>,
    },

    Or {
        a: Box<Expression>,
        b: Box<Expression>,
    },

    And {
        a: Box<Expression>,
        b: Box<Expression>,
    },

    Grouping {
        e: Box<Expression>,
    },
}

impl std::fmt::Display for Expression {
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
            Self::Binary { op, a, b } => write!(f, "({op} {a} {b})"),
            Self::Or { a, b } => write!(f, "(OR {a} {b})"),
            Self::And { a, b } => write!(f, "(AND {a} {b})"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_display() {
        let ast = Expression::Binary {
            op: OpTwo::Star,
            a: Box::new(Expression::Unary {
                op: OpOne::Minus,
                e: Box::new(Expression::Literal {
                    l: Literal::Number { n: 123_f64 },
                }),
            }),
            b: Box::new(Expression::Grouping {
                e: Box::new(Expression::Literal {
                    l: Literal::Number { n: 45.67 },
                }),
            }),
        };

        assert_eq!(format!("{ast}"), "(* (- 123) (group 45.67))");
    }
}
