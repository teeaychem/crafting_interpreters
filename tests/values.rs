#[cfg(test)]
mod value_conversion {
    use loxy_lib::interpreter::evaluation::value::Value;

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
