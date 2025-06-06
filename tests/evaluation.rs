#[cfg(test)]
mod evaluation {
    use loxy_lib::{
        Interpreter,
        interpreter::{
            ast::expression::{Expression, OpOne, OpTwo},
            environment::Env,
            parser::value::Value,
        },
    };

    #[test]
    fn basic_negation() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_global_handle();

        let number = Expression::from(64.0);

        let number_negation = Expression::mk_unary(OpOne::Minus, number);

        assert_eq!(
            interpreter.eval(&number_negation, &env),
            Ok(Value::from(-64.0))
        );

        let string = Expression::from("64");

        let string_negation = Expression::mk_unary(OpOne::Minus, string);

        assert_eq!(
            interpreter.eval(&string_negation, &env),
            Ok(Value::from(-64.0))
        );
    }

    #[test]
    fn basic_arithmetic() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_global_handle();

        let a_value = 64.0;
        let b_value = 32.0;

        let a = Expression::from(a_value);
        let b = Expression::from(b_value);

        let addition = Expression::mk_binary(OpTwo::Star, a, b);

        assert_eq!(
            interpreter.eval(&addition, &env),
            Ok(Value::from(a_value * b_value))
        );
    }

    #[test]
    fn basic_string() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_global_handle();

        let a = Expression::from("a ");
        let b = Expression::from("string");

        let addition = Expression::mk_binary(OpTwo::Plus, a, b);

        assert_eq!(
            interpreter.eval(&addition, &env),
            Ok(Value::from("a string"))
        );
    }

    #[test]
    fn basic_comparison() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_global_handle();

        let a_value = 64.0;
        let b_value = 32.0;

        let gt = Expression::mk_binary(
            OpTwo::Gt,
            Expression::from(a_value),
            Expression::from(b_value),
        );
        let leq = Expression::mk_binary(
            OpTwo::Leq,
            Expression::from(a_value),
            Expression::from(b_value),
        );

        assert_eq!(interpreter.eval(&gt, &env), Ok(Value::from(true)));
        assert_eq!(interpreter.eval(&leq, &env), Ok(Value::from(false)));
    }

    #[test]
    fn basic_equality() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_global_handle();

        let a_value = 64.0;
        let b_value = 32.0;

        let eq_self = Expression::mk_binary(
            OpTwo::Eq,
            Expression::from(a_value),
            Expression::from(a_value),
        );

        let eq_same = Expression::mk_binary(
            OpTwo::Eq,
            Expression::from(Expression::from("a")),
            Expression::from(Expression::from("a")),
        );

        let neq = Expression::mk_binary(
            OpTwo::Eq,
            Expression::from(a_value),
            Expression::from(b_value),
        );

        let neq_different_types =
            Expression::mk_binary(OpTwo::Eq, Expression::from("64.0"), Expression::from(64.0));

        assert_eq!(interpreter.eval(&eq_self, &env), Ok(Value::from(true)));
        assert_eq!(interpreter.eval(&eq_same, &env), Ok(Value::from(true)));
        assert_eq!(interpreter.eval(&neq, &env), Ok(Value::from(false)));
        assert_eq!(
            interpreter.eval(&neq_different_types, &env),
            Ok(Value::from(false))
        );
    }
}
