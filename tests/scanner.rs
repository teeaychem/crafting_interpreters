#[cfg(test)]
mod scanner {
    use loxy_lib::{
        Scanner,
        interpreter::{
            location::Location,
            scanner::token::{Token, TokenKind},
        },
    };

    #[test]
    fn scanner_basic_numeric() {
        let mut scanner = Scanner::default();
        scanner.scan("1 0.23\n  1.23");

        assert_eq!(
            scanner.tokens,
            vec![
                Token {
                    kind: TokenKind::Number { literal: 1.0 },
                    location: Location::default()
                },
                Token {
                    kind: TokenKind::Number { literal: 0.23 },
                    location: Location::new(0, 2)
                },
                Token {
                    kind: TokenKind::Number { literal: 1.23 },
                    location: Location::new(1, 2)
                }
            ]
        );
    }

    #[test]
    fn scanner_basic_keyword() {
        let mut scanner = Scanner::default();
        scanner.scan("not");
        scanner.scan(" ");
        scanner.scan("(true and perhaps false)");

        assert_eq!(
            scanner.tokens,
            vec![
                Token {
                    kind: TokenKind::Identifier {
                        id: "not".to_string()
                    },
                    location: Location::default()
                },
                Token {
                    kind: TokenKind::ParenLeft,
                    location: Location::new(0, 4)
                },
                Token {
                    kind: TokenKind::True,
                    location: Location::new(0, 5)
                },
                Token {
                    kind: TokenKind::And,
                    location: Location::new(0, 10)
                },
                Token {
                    kind: TokenKind::Identifier {
                        id: "perhaps".to_string()
                    },
                    location: Location::new(0, 14)
                },
                Token {
                    kind: TokenKind::False,
                    location: Location::new(0, 22)
                },
                Token {
                    kind: TokenKind::ParenRight,
                    location: Location::new(0, 27)
                }
            ]
        );
    }
}
