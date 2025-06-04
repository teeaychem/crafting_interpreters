#[derive(Debug, Clone, Copy)]
pub enum OpOne {
    Minus,
    Bang,
}

impl std::fmt::Display for OpOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),

            Self::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpTwo {
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

impl std::fmt::Display for OpTwo {
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
