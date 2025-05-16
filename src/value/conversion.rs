use super::{Value, ValueError};

impl Value {
    pub fn to_boolean(self) -> Result<Self, ValueError> {
        match self {
            Value::Null => Err(ValueError::InvalidConversion),

            Value::Boolean { b } => Ok(self),

            Self::String { s } => match s.to_lowercase().parse::<bool>() {
                Ok(v) => Ok(Value::from(v)),

                Err(_) => Err(ValueError::InvalidConversion),
            },

            Self::Numeric { n } => match n {
                m if m < 1.0 => Ok(Value::from(false)),

                _ => Ok(Value::from(true)),
            },
        }
    }

    pub fn parity(a: &mut Value, b: &mut Value) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn boolean() {
        assert_eq!(Err(ValueError::InvalidConversion), Value::Null.to_boolean());

        let true_conversions = [
            Value::from(true),
            Value::from(1.0),
            Value::from(1.5),
            Value::from(2_f64.powf(32.0)),
            Value::from("true"),
            Value::from("True"),
        ];

        for true_coversion in true_conversions {
            assert_eq!(true_coversion.to_boolean(), Ok(Value::from(true)));
        }

        let false_conversions = [
            Value::from(false),
            Value::from(0.0),
            Value::from(0.5),
            Value::from("false"),
            Value::from("False"),
        ];

        for false_coversion in false_conversions {
            assert_eq!(false_coversion.to_boolean(), Ok(Value::from(false)));
        }
    }
}
