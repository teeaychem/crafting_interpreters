use crate::interpreter::{
    Scanner,
    location::Location,
    scanner::token::{Tkn, TknKind},
};

#[test]
fn scanner_basic_numeric() {
    let mut scanner = Scanner::default();
    scanner.scan("1 0.23\n  1.23");

    assert_eq!(
        scanner.tokens,
        vec![
            Tkn {
                kind: TknKind::Number { literal: 1.0 },
                location: Location::default()
            },
            Tkn {
                kind: TknKind::Number { literal: 0.23 },
                location: Location::new(0, 2)
            },
            Tkn {
                kind: TknKind::Number { literal: 1.23 },
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
                kind: TknKind::Identifier {
                    id: "not".to_string()
                },
                location: Location::default()
            },
            Tkn {
                kind: TknKind::ParenLeft,
                location: Location::new(0, 4)
            },
            Tkn {
                kind: TknKind::True,
                location: Location::new(0, 5)
            },
            Tkn {
                kind: TknKind::And,
                location: Location::new(0, 10)
            },
            Tkn {
                kind: TknKind::Identifier {
                    id: "perhaps".to_string()
                },
                location: Location::new(0, 14)
            },
            Tkn {
                kind: TknKind::False,
                location: Location::new(0, 22)
            },
            Tkn {
                kind: TknKind::ParenRight,
                location: Location::new(0, 27)
            }
        ]
    );
}
