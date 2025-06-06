use super::{Assignment, EvalErr};

impl Assignment {
    pub fn to_boolean(self) -> Result<Self, EvalErr> {
        match self {
            Assignment::Nil => Ok(Assignment::from(false)),

            Assignment::Boolean { .. } => Ok(self),

            _ => Ok(Assignment::from(true)),
        }
    }

    pub fn to_numeric(self) -> Result<Self, EvalErr> {
        match self {
            Assignment::Nil => Err(EvalErr::InvalidConversion),

            Assignment::Boolean { b } => Err(EvalErr::InvalidConversion),

            Self::String { s } => match s.parse::<f64>() {
                Ok(v) => Ok(Assignment::from(v)),

                Err(_) => Err(EvalErr::InvalidConversion),
            },

            Self::Numeric { .. } => Ok(self),
        }
    }

    pub fn to_string(self) -> Result<Self, EvalErr> {
        let value = match self {
            Assignment::Nil => return Err(EvalErr::InvalidConversion),

            Assignment::Boolean { b } => match b {
                true => Assignment::from("true"),
                false => Assignment::from("false"),
            },

            Self::String { .. } => self,

            Self::Numeric { n } => Assignment::from(n.to_string()),
        };

        Ok(value)
    }
}
