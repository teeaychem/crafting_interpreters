use crate::interpreter::{
    Scanner,
    location::Location,
    scanner::token::{Tkn, TknK},
};

#[test]
fn scanner_basic_numeric() {
    let mut scanner = Scanner::default();
    scanner.scan("1 0.23\n  1.23");

    assert_eq!(
        scanner.tokens,
        vec![
            Tkn {
                kind: TknK::Number { literal: 1.0 },
                location: Location::default()
            },
            Tkn {
                kind: TknK::Number { literal: 0.23 },
                location: Location::new(0, 2)
            },
            Tkn {
                kind: TknK::Number { literal: 1.23 },
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
            Tkn {
                kind: TknK::Identifier {
                    id: "not".to_string()
                },
                location: Location::default()
            },
            Tkn {
                kind: TknK::ParenL,
                location: Location::new(0, 4)
            },
            Tkn {
                kind: TknK::True,
                location: Location::new(0, 5)
            },
            Tkn {
                kind: TknK::And,
                location: Location::new(0, 10)
            },
            Tkn {
                kind: TknK::Identifier {
                    id: "perhaps".to_string()
                },
                location: Location::new(0, 14)
            },
            Tkn {
                kind: TknK::False,
                location: Location::new(0, 22)
            },
            Tkn {
                kind: TknK::ParenR,
                location: Location::new(0, 27)
            }
        ]
    );
}
