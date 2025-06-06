mod builders;
mod operators;

pub use operators::{OpOne, OpTwo};

use crate::interpreter::environment::EnvHandle;

use super::{
    identifier::Identifier,
    statement::{Statement, Statements},
};

#[derive(Clone, Debug)]
pub enum ExprB {
    Nil,

    Boolean {
        b: bool,
    },

    Numeric {
        n: f64,
    },

    String {
        s: String,
    },

    Lambda {
        env: EnvHandle,
        params: Vec<Identifier>,
        body: Statements,
    },
}

impl PartialEq for ExprB {
    fn eq(&self, other: &Self) -> bool {
        use ExprB::*;

        match (self, other) {
            (Nil, _) => false,

            (_, Nil) => false,

            (Boolean { b: l }, Boolean { b: r }) => l == r,

            (Numeric { n: l }, Numeric { n: r }) => l == r,

            (String { s: l }, String { s: r }) => l == r,

            _ => false,
        }
    }
}

impl std::fmt::Display for ExprB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),

            Self::Boolean { b } => write!(f, "{b}"),

            Self::Numeric { n } => write!(f, "{n}"),

            Self::String { s } => write!(f, "{s}"),

            Self::Lambda { env, params, body } => write!(f, "λ"),
        }
    }
}

impl ExprB {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Numeric { .. } => true,

            Self::String { .. } => true,

            Self::Lambda { .. } => true,

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

#[derive(Debug, Clone, PartialEq)]
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
        caller: Box<Expr>,
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

            Self::Call { caller, args } => write!(
                f,
                "{}({})",
                caller,
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
