#[cfg(test)]
mod evaluation {
    use loxy_lib::{interpreter::{ast::expression::{Expression, OpB, OpU}, parser::value::Value}, Interpreter};



    #[test]
    fn basic_negation() {
        let mut interpreter = Interpreter::new();
        let number = Expression::from(64.0);

        let number_negation = Expression::mk_unary(OpU::Minus, number);

        assert_eq!(
            interpreter.evaluate(&number_negation),
            Ok(Value::from(-64.0))
        );

        let string = Expression::from("64");

        let string_negation = Expression::mk_unary(OpU::Minus, string);

        assert_eq!(
            interpreter.evaluate(&string_negation),
            Ok(Value::from(-64.0))
        );
    }

    #[test]
    fn basic_arithmetic() {
        let mut interpreter = Interpreter::new();

        let a_value = 64.0;
        let b_value = 32.0;

        let a = Expression::from(a_value);
        let b = Expression::from(b_value);

        let addition = Expression::mk_binary(OpB::Star, a, b);

        assert_eq!(
            interpreter.evaluate(&addition),
            Ok(Value::from(a_value * b_value))
        );
    }

    #[test]
    fn basic_string() {
        let mut interpreter = Interpreter::new();

        let a = Expression::from("a ");
        let b = Expression::from("string");

        let addition = Expression::mk_binary(OpB::Plus, a, b);

        assert_eq!(interpreter.evaluate(&addition), Ok(Value::from("a string")));
    }

    #[test]
    fn basic_comparison() {
        let mut interpreter = Interpreter::new();

        let a_value = 64.0;
        let b_value = 32.0;

        let gt = Expression::mk_binary(
            OpB::Gt,
            Expression::from(a_value),
            Expression::from(b_value),
        );
        let leq = Expression::mk_binary(
            OpB::Leq,
            Expression::from(a_value),
            Expression::from(b_value),
        );

        assert_eq!(interpreter.evaluate(&gt), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&leq), Ok(Value::from(false)));
    }

    #[test]
    fn basic_equality() {
        let mut interpreter = Interpreter::new();

        let a_value = 64.0;
        let b_value = 32.0;

        let eq_self = Expression::mk_binary(
            OpB::Eq,
            Expression::from(a_value),
            Expression::from(a_value),
        );

        let eq_same = Expression::mk_binary(
            OpB::Eq,
            Expression::from(Expression::from("a")),
            Expression::from(Expression::from("a")),
        );

        let neq = Expression::mk_binary(
            OpB::Eq,
            Expression::from(a_value),
            Expression::from(b_value),
        );

        let neq_different_types =
            Expression::mk_binary(OpB::Eq, Expression::from("64.0"), Expression::from(64.0));

        assert_eq!(interpreter.evaluate(&eq_self), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&eq_same), Ok(Value::from(true)));
        assert_eq!(interpreter.evaluate(&neq), Ok(Value::from(false)));
        assert_eq!(
            interpreter.evaluate(&neq_different_types),
            Ok(Value::from(false))
        );
    }
}
