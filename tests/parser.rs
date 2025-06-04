#[cfg(test)]
mod parser {
    use loxy_lib::{Parser, Scanner};

    #[test]
    fn simple() {
        let mut scanner = Scanner::default();
        let input = "! true == false";

        scanner.scan(input);

        let mut parser = Parser::from(scanner);
        let expr = parser.expression();

        match expr {
            Ok(expr) => assert_eq!(format!("{expr}"), "(== (! true) false)"),

            Err(_) => panic!("Failed to parse {input}"),
        }
    }

    #[test]
    fn arithmetic() {
        let mut scanner = Scanner::default();
        let input = "4 / 3 * - 2";

        scanner.scan(input);

        let mut parser = Parser::from(scanner);
        let expr = parser.expression();

        match expr {
            Ok(expr) => assert_eq!(format!("{expr}"), "(/ 4 (* 3 (- 2)))"),

            Err(_) => panic!("Failed to parse {input}"),
        }
    }

    #[test]
    fn sync() {
        let mut scanner = Scanner::default();
        let input = "4 / ; + 2 2; 2 + 2";

        scanner.scan(input);

        let mut parser = Parser::from(scanner);
        let expr = parser.expression();

        match expr {
            Ok(_) => panic!("Expected parse error"),

            Err(_) => loop {
                match parser.expression() {
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
}
