#[cfg(test)]
mod value_conversion {
    use loxy_lib::interpreter::evaluation::value::Assignment;

    #[test]
    fn boolean() {
        let true_conversions = [
            Assignment::from(true),
            Assignment::from(0.0),
            Assignment::from(1.5),
            Assignment::from(2_f64.powf(32.0)),
            Assignment::from("true"),
            Assignment::from("False"),
        ];

        for true_coversion in true_conversions {
            assert_eq!(true_coversion.to_boolean(), Ok(Assignment::from(true)));
        }

        let false_conversions = [Assignment::from(false), Assignment::Nil];

        for false_coversion in false_conversions {
            assert_eq!(false_coversion.to_boolean(), Ok(Assignment::from(false)));
        }
    }

    #[test]
    fn numeric() {
        assert_eq!(
            Ok(Assignment::from(64.0)),
            Assignment::from("64").to_numeric()
        );
        assert_eq!(
            Ok(Assignment::from(-64.0)),
            Assignment::from("-64").to_numeric()
        );
    }

    #[test]
    fn string() {
        assert_eq!(
            Ok(Assignment::from("64")),
            Assignment::from(64.0).to_string()
        );
        assert_eq!(
            Ok(Assignment::from("true")),
            Assignment::from(true).to_string()
        );
    }
}
