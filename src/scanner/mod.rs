use std::{iter::Peekable, str::Chars};

use crate::{
    location::Location,
    scanner::token::{Token, TokenError, TokenKind, Tokens},
};

pub mod token;

pub struct Scanner {
    pub location: Location,
    pub tokens: Tokens,
}

impl Default for Scanner {
    fn default() -> Self {
        Scanner {
            location: Location::default(),
            tokens: Vec::default(),
        }
    }
}

impl Scanner {
    fn take_whitespace(&mut self, chars: &mut Peekable<Chars<'_>>) {
        'whitespace_loop: loop {
            if let Some(c) = chars.peek() {
                match c {
                    '\n' => self.location.newline(),

                    w if w.is_whitespace() => self.location.advance_col(1),

                    _ => break 'whitespace_loop,
                }

                chars.next();
            } else {
                break 'whitespace_loop;
            }
        }
    }

    fn take_string(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<(), TokenError> {
        chars.next();
        let mut literal = String::default();

        while let Some(d) = chars.peek() {
            match d {
                '"' => {
                    let length = literal.len() + 2;

                    self.note_token(
                        TokenKind::String {
                            literal: std::mem::take(&mut literal),
                        },
                        length,
                    );
                    chars.next();
                    break;
                }

                '\n' => return Err(TokenError::MultilineString),

                _ => {
                    literal.push(*d);
                    chars.next();
                }
            }
        }

        Ok(())
    }

    fn take_comment(&mut self, chars: &mut Peekable<Chars<'_>>) {
        chars.next();
        while chars.peek().is_some_and(|d| *d != '\n') {
            chars.next();
        }
        self.location.newline();
    }

    fn take_numeric(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<(), TokenError> {
        let mut number = String::default();

        while let Some(m) = chars.peek() {
            if m.is_numeric() || *m == '.' {
                number.push(*m);
                chars.next();
            } else {
                break;
            }
        }

        if let Some('.') = number.chars().last() {
            return Err(TokenError::TrailingDot);
        }

        self.note_token(
            TokenKind::Number {
                literal: number.parse().unwrap(),
            },
            number.len(),
        );

        Ok(())
    }

    fn take_alphabetic(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<(), TokenError> {
        let mut alphabetic = String::default();
        while let Some(b) = chars.peek() {
            if b.is_alphabetic() || *b == '_' {
                alphabetic.push(*b);
                chars.next();
            } else {
                break;
            }
        }

        let instance = match alphabetic.as_str() {
            "and" => TokenKind::And,

            "class" => TokenKind::Class,

            "else" => TokenKind::Else,

            "false" => TokenKind::False,

            "for" => TokenKind::For,

            "fun" => TokenKind::Fun,

            "if" => TokenKind::If,

            "nil" => TokenKind::Nil,

            "or" => TokenKind::Or,

            "print" => TokenKind::Print,

            "return" => TokenKind::Return,

            "super" => TokenKind::Super,

            "this" => TokenKind::This,

            "true" => TokenKind::True,

            "var" => TokenKind::Var,

            "while" => TokenKind::While,

            non_keyword => TokenKind::Identifier {
                id: non_keyword.to_owned(),
            },
        };

        self.note_token(instance, alphabetic.len());

        Ok(())
    }

    fn take_characters(
        &mut self,
        chars: &mut Peekable<Chars<'_>>,
        count: usize,
        instance: TokenKind,
    ) -> Result<(), TokenError> {
        self.note_token(instance, count);
        for _ in 0..count {
            chars.next();
        }

        Ok(())
    }
}

impl Scanner {
    pub fn scan<I: AsRef<str>>(&mut self, s: I) {
        let mut chars = s.as_ref().chars().peekable();
        while let Ok(true) = self.take_token(&mut chars) {}
    }

    fn scan_punctuation() {}

    fn note_token(&mut self, instance: TokenKind, advance: usize) {
        self.tokens.push(Token {
            kind: instance,
            location: self.location,
        });
        self.location.advance_col(advance);
    }

    pub fn take_token(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<bool, TokenError> {
        self.take_whitespace(chars);

        match chars.peek() {
            None => Ok(false),

            Some(c) => {
                match c {
                    '"' => self.take_string(chars)?,

                    '/' => {
                        chars.next();
                        if let Some('/') = chars.peek() {
                            self.take_comment(chars);
                        } else {
                            self.note_token(TokenKind::Slash, 1);
                        }
                    }

                    '(' => self.take_characters(chars, 1, TokenKind::ParenLeft)?,

                    ')' => self.take_characters(chars, 1, TokenKind::ParenRight)?,

                    '{' => self.take_characters(chars, 1, TokenKind::BraceLeft)?,

                    '}' => self.take_characters(chars, 1, TokenKind::BraceRight)?,

                    ',' => self.take_characters(chars, 1, TokenKind::Comma)?,

                    '.' => self.take_characters(chars, 1, TokenKind::Dot)?,

                    '-' => self.take_characters(chars, 1, TokenKind::Minus)?,

                    '+' => self.take_characters(chars, 1, TokenKind::Plus)?,

                    ';' => self.take_characters(chars, 1, TokenKind::Semicolon)?,

                    '*' => self.take_characters(chars, 1, TokenKind::Star)?,

                    '!' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TokenKind::BangEqual)?
                        } else {
                            self.take_characters(chars, 0, TokenKind::Bang)?
                        }
                    }

                    '=' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TokenKind::EqualEqual)?
                        } else {
                            self.take_characters(chars, 0, TokenKind::Equal)?
                        }
                    }

                    '<' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TokenKind::LessEqual)?
                        } else {
                            self.take_characters(chars, 0, TokenKind::Less)?
                        }
                    }

                    '>' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TokenKind::GreaterEqual)?
                        } else {
                            self.take_characters(chars, 0, TokenKind::Greater)?
                        }
                    }

                    numeric if numeric.is_numeric() => self.take_numeric(chars)?,

                    alphabetic if alphabetic.is_alphabetic() => self.take_alphabetic(chars)?,

                    unrecognised => {
                        return Err(TokenError::Unrecognised {
                            character: *unrecognised,
                        });
                    }
                }

                Ok(true)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

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
                    kind: Identifier {
                        id: "not".to_string()
                    },
                    location: Location::default()
                },
                Token {
                    kind: ParenLeft,
                    location: Location::new(0, 4)
                },
                Token {
                    kind: True,
                    location: Location::new(0, 5)
                },
                Token {
                    kind: And,
                    location: Location::new(0, 10)
                },
                Token {
                    kind: Identifier {
                        id: "perhaps".to_string()
                    },
                    location: Location::new(0, 14)
                },
                Token {
                    kind: False,
                    location: Location::new(0, 22)
                },
                Token {
                    kind: ParenRight,
                    location: Location::new(0, 27)
                }
            ]
        );
    }
}
