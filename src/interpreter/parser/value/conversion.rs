use super::{Value, ValueError};

impl Value {
    pub fn to_boolean(self) -> Result<Self, ValueError> {
        match self {
            Value::Nil => Ok(Value::from(false)),

            Value::Boolean { .. } => Ok(self),

            _ => Ok(Value::from(true)),
        }
    }

    pub fn to_numeric(self) -> Result<Self, ValueError> {
        match self {
            Value::Nil => Err(ValueError::InvalidConversion),

            Value::Boolean { b } => Err(ValueError::InvalidConversion),

            Self::String { s } => match s.parse::<f64>() {
                Ok(v) => Ok(Value::from(v)),

                Err(_) => Err(ValueError::InvalidConversion),
            },

            Self::Numeric { .. } => Ok(self),
        }
    }

    pub fn to_string(self) -> Result<Self, ValueError> {
        let value = match self {
            Value::Nil => return Err(ValueError::InvalidConversion),

            Value::Boolean { b } => match b {
                true => Value::from("true"),
                false => Value::from("false"),
            },

            Self::String { .. } => self,

            Self::Numeric { n } => Value::from(n.to_string()),
        };

        Ok(value)
    }

    pub fn parity(a: &mut Value, b: &mut Value) {}
}
