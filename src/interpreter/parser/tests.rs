use crate::interpreter::{TreeWalker, environment::Env};

#[test]
fn simple() {
    let input = "! true == false";

    let mut parser = TreeWalker::default();
    parser.scan(input);

    let env = Env::fresh_std_env();
    let expr = parser.expression(&env);

    match expr {
        Ok(expr) => assert_eq!(format!("{expr}"), "(== (! true) false)"),

        Err(_) => panic!("Failed to parse {input}"),
    }
}

#[test]
fn arithmetic() {
    let input = "4 / 3 * - 2";

    let mut parser = TreeWalker::default();
    parser.scan(input);

    let env = Env::fresh_std_env();

    let expr = parser.expression(&env);

    match expr {
        Ok(expr) => assert_eq!(format!("{expr}"), "(/ 4 (* 3 (- 2)))"),

        Err(_) => panic!("Failed to parse {input}"),
    }
}

#[test]
fn sync() {
    let input = "4 / ; + 2 2; 2 + 2";

    let mut parser = TreeWalker::default();
    parser.scan(input);

    let env = Env::fresh_std_env();

    let expr = parser.expression(&env);

    match expr {
        Ok(_) => panic!("Expected parse error"),

        Err(_) => loop {
            match parser.expression(&env) {
                Ok(expr) => {
                    assert_eq!(format!("{expr}"), "(+ 2 2)");
                    break;
                }
                Err(_) => {
                    if parser.token().is_none() {
                        panic!("Failed to sync before EOF");
                    }

                    parser.syncronise();
                }
            }
        },
    }
}
