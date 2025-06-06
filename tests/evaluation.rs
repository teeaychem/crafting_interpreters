#[cfg(test)]
mod evaluation {
    use loxy_lib::{
        Interpreter,
        interpreter::{
            ast::expression::{Expr, ExprB, OpOne, OpTwo},
            environment::Env,
        },
    };

    #[test]
    fn basic_negation() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_std_env();

        let number = Expr::from(64.0);

        let number_negation = Expr::mk_unary(OpOne::Minus, number);

        assert_eq!(
            interpreter.eval(&number_negation, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_numeric(-64.0))
        );

        let string = Expr::from("64");

        let string_negation = Expr::mk_unary(OpOne::Minus, string);

        assert_eq!(
            interpreter.eval(&string_negation, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_numeric(-64.0))
        );
    }

    #[test]
    fn basic_arithmetic() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_std_env();

        let a_value = 64.0;
        let b_value = 32.0;

        let a = Expr::from(a_value);
        let b = Expr::from(b_value);

        let addition = Expr::mk_binary(OpTwo::Star, a, b);

        assert_eq!(
            interpreter.eval(&addition, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_numeric(a_value * b_value))
        );
    }

    #[test]
    fn basic_string() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_std_env();

        let a = Expr::from("a ");
        let b = Expr::from("string");

        let addition = Expr::mk_binary(OpTwo::Plus, a, b);

        assert_eq!(
            interpreter.eval(&addition, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_string("a string".to_owned()))
        );
    }

    #[test]
    fn basic_comparison() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_std_env();

        let a_value = 64.0;
        let b_value = 32.0;

        let gt = Expr::mk_binary(OpTwo::Gt, Expr::from(a_value), Expr::from(b_value));
        let leq = Expr::mk_binary(OpTwo::Leq, Expr::from(a_value), Expr::from(b_value));

        assert_eq!(
            interpreter.eval(&gt, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_bool(true))
        );
        assert_eq!(
            interpreter.eval(&leq, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_bool(false))
        );
    }

    #[test]
    fn basic_equality() {
        let interpreter = Interpreter::default();
        let env = Env::fresh_std_env();

        let a_value = 64.0;
        let b_value = 32.0;

        let eq_self = Expr::mk_binary(OpTwo::Eq, Expr::from(a_value), Expr::from(a_value));

        let eq_same = Expr::mk_binary(
            OpTwo::Eq,
            Expr::from(Expr::from("a")),
            Expr::from(Expr::from("a")),
        );

        let neq = Expr::mk_binary(OpTwo::Eq, Expr::from(a_value), Expr::from(b_value));

        let neq_different_types = Expr::mk_binary(OpTwo::Eq, Expr::from("64.0"), Expr::from(64.0));

        assert_eq!(
            interpreter.eval(&eq_self, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_bool(true))
        );
        assert_eq!(
            interpreter.eval(&eq_same, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_bool(true))
        );
        assert_eq!(
            interpreter.eval(&neq, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_bool(false))
        );
        assert_eq!(
            interpreter.eval(&neq_different_types, &env, &mut std::io::stdout()),
            Ok(ExprB::mk_bool(false))
        );
    }
}
