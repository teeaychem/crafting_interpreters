use crate::interpreter::{TreeWalker, environment::Env};

#[test]
fn simple() {
    let input = "! true == false";

    let mut parser = TreeWalker::default();
    parser.scan(input);

    let env = Env::fresh_std_env();
    let expr = parser.expression(&env);

    assert!(expr.is_ok());

    let expr = expr.unwrap();

    assert_eq!(format!("{expr}"), "(== (! true) false)");
}

#[test]
fn arithmetic() {
    let input = "4 / 3 * - 2";

    let mut parser = TreeWalker::default();
    parser.scan(input);

    let env = Env::fresh_std_env();

    let expr = parser.expression(&env);

    assert!(expr.is_ok());

    let expr = expr.unwrap();

    assert_eq!(format!("{expr}"), "(/ 4 (* 3 (- 2)))");
}

#[test]
fn sync() {
    let input = "4 / ; + 2 2; 2 + 2";

    let mut parser = TreeWalker::default();
    parser.scan(input);

    let env = Env::fresh_std_env();

    let expr = parser.expression(&env);

    assert!(expr.is_err());

    loop {
        match parser.expression(&env) {
            Ok(expr) => return assert_eq!(format!("{expr}"), "(+ 2 2)"),

            Err(_) => {
                if parser.token().is_none() {
                    panic!("Failed to sync before EOF");
                }

                parser.syncronise();
            }
        }
    }
}
