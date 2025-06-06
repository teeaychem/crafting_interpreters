mod builders;
mod operators;

pub use operators::{OpOne, OpTwo};

use super::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprB {
    Nil,

    Boolean { b: bool },

    Numeric { n: f64 },

    String { s: String },
}

impl std::fmt::Display for ExprB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),

            Self::Boolean { b } => write!(f, "{b}"),

            Self::Numeric { n } => write!(f, "{n}"),

            Self::String { s } => write!(f, "{s}"),
        }
    }
}

impl ExprB {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Numeric { n } => true,

            Self::String { s } => true,

            Self::Boolean { b } => *b,

            Self::Nil => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        !self.is_truthy()
    }
}

impl ExprB {
    pub fn mk_bool(b: bool) -> ExprB {
        Self::Boolean { b }
    }

    pub fn mk_numeric(n: f64) -> ExprB {
        Self::Numeric { n }
    }

    pub fn mk_string(s: String) -> ExprB {
        Self::String { s }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Empty,

    Basic(ExprB),

    Identifier {
        id: Identifier,
    },

    Assignment {
        id: Box<Expr>,
        e: Box<Expr>,
    },

    Unary {
        op: OpOne,
        e: Box<Expr>,
    },

    Binary {
        op: OpTwo,
        a: Box<Expr>,
        b: Box<Expr>,
    },

    Or {
        a: Box<Expr>,
        b: Box<Expr>,
    },

    And {
        a: Box<Expr>,
        b: Box<Expr>,
    },

    Grouping {
        e: Box<Expr>,
    },

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "nil"),

            Self::Basic(bexpr) => write!(f, "{bexpr}"),

            Self::Identifier { id: l } => write!(f, "{l}"),

            Expr::Assignment {
                id: name,
                e: assignment,
            } => write!(f, "{name} = {assignment}"),

            Self::Grouping { e } => write!(f, "(group {e})"),

            Self::Unary { op, e } => write!(f, "({op} {e})"),

            Self::Binary { op, a, b } => write!(f, "({op} {a} {b})"),

            Self::Or { a, b } => write!(f, "(OR {a} {b})"),

            Self::And { a, b } => write!(f, "(AND {a} {b})"),

            Self::Call { callee, args } => write!(
                f,
                "{}({})",
                callee,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_display() {
        let ast = Expr::Binary {
            op: OpTwo::Star,

            a: Box::new(Expr::Unary {
                op: OpOne::Minus,
                e: Box::new(Expr::Basic(ExprB::Numeric { n: 123_f64 })),
            }),

            b: Box::new(Expr::Grouping {
                e: Box::new(Expr::Basic(ExprB::Numeric { n: 45.67 })),
            }),
        };

        assert_eq!(format!("{ast}"), "(* (- 123) (group 45.67))");
    }
}
