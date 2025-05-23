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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn boolean() {
        let true_conversions = [
            Value::from(true),
            Value::from(0.0),
            Value::from(1.5),
            Value::from(2_f64.powf(32.0)),
            Value::from("true"),
            Value::from("False"),
        ];

        for true_coversion in true_conversions {
            assert_eq!(true_coversion.to_boolean(), Ok(Value::from(true)));
        }

        let false_conversions = [Value::from(false), Value::Nil];

        for false_coversion in false_conversions {
            assert_eq!(false_coversion.to_boolean(), Ok(Value::from(false)));
        }
    }

    #[test]
    fn numeric() {
        assert_eq!(Ok(Value::from(64.0)), Value::from("64").to_numeric());
        assert_eq!(Ok(Value::from(-64.0)), Value::from("-64").to_numeric());
    }

    #[test]
    fn string() {
        assert_eq!(Ok(Value::from("64")), Value::from(64.0).to_string());
        assert_eq!(Ok(Value::from("true")), Value::from(true).to_string());
    }
}
