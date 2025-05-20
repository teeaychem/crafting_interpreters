use std::{iter::Peekable, str::Chars};

use crate::{
    location::Location,
    scanner::token::{Token, TokenError, TokenInstance, Tokens},
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
                        TokenInstance::String {
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
            TokenInstance::Number {
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
            "and" => TokenInstance::And,

            "class" => TokenInstance::Class,

            "else" => TokenInstance::Else,

            "false" => TokenInstance::False,

            "for" => TokenInstance::For,

            "fun" => TokenInstance::Fun,

            "if" => TokenInstance::If,

            "nil" => TokenInstance::Nil,

            "or" => TokenInstance::Or,

            "print" => TokenInstance::Print,

            "return" => TokenInstance::Return,

            "super" => TokenInstance::Super,

            "this" => TokenInstance::This,

            "true" => TokenInstance::True,

            "var" => TokenInstance::Var,

            "while" => TokenInstance::While,

            non_keyword => TokenInstance::Identifier {
                literal: non_keyword.to_owned(),
            },
        };

        self.note_token(instance, alphabetic.len());

        Ok(())
    }

    fn take_characters(
        &mut self,
        chars: &mut Peekable<Chars<'_>>,
        count: usize,
        instance: TokenInstance,
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

    fn note_token(&mut self, instance: TokenInstance, advance: usize) {
        self.tokens.push(Token {
            instance,
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
                            self.note_token(TokenInstance::Slash, 1);
                        }
                    }

                    '(' => self.take_characters(chars, 1, TokenInstance::ParenLeft)?,

                    ')' => self.take_characters(chars, 1, TokenInstance::ParenRight)?,

                    '{' => self.take_characters(chars, 1, TokenInstance::BraceLeft)?,

                    '}' => self.take_characters(chars, 1, TokenInstance::BraceRight)?,

                    ',' => self.take_characters(chars, 1, TokenInstance::Comma)?,

                    '.' => self.take_characters(chars, 1, TokenInstance::Dot)?,

                    '-' => self.take_characters(chars, 1, TokenInstance::Minus)?,

                    '+' => self.take_characters(chars, 1, TokenInstance::Plus)?,

                    ';' => self.take_characters(chars, 1, TokenInstance::Semicolon)?,

                    '*' => self.take_characters(chars, 1, TokenInstance::Star)?,

                    '!' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TokenInstance::BangEqual)?
                        } else {
                            self.take_characters(chars, 0, TokenInstance::Bang)?
                        }
                    }

                    '=' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TokenInstance::EqualEqual)?
                        } else {
                            self.take_characters(chars, 0, TokenInstance::Equal)?
                        }
                    }

                    '<' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TokenInstance::LessEqual)?
                        } else {
                            self.take_characters(chars, 0, TokenInstance::Less)?
                        }
                    }

                    '>' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TokenInstance::GreaterEqual)?
                        } else {
                            self.take_characters(chars, 0, TokenInstance::Greater)?
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
    use TokenInstance::*;

    #[test]
    fn scanner_basic_numeric() {
        let mut scanner = Scanner::default();
        scanner.scan("1 0.23\n  1.23");

        assert_eq!(
            scanner.tokens,
            vec![
                Token {
                    instance: TokenInstance::Number { literal: 1.0 },
                    location: Location::default()
                },
                Token {
                    instance: TokenInstance::Number { literal: 0.23 },
                    location: Location::new(0, 2)
                },
                Token {
                    instance: TokenInstance::Number { literal: 1.23 },
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
                    instance: Identifier {
                        literal: "not".to_string()
                    },
                    location: Location::default()
                },
                Token {
                    instance: ParenLeft,
                    location: Location::new(0, 4)
                },
                Token {
                    instance: True,
                    location: Location::new(0, 5)
                },
                Token {
                    instance: And,
                    location: Location::new(0, 10)
                },
                Token {
                    instance: Identifier {
                        literal: "perhaps".to_string()
                    },
                    location: Location::new(0, 14)
                },
                Token {
                    instance: False,
                    location: Location::new(0, 22)
                },
                Token {
                    instance: ParenRight,
                    location: Location::new(0, 27)
                }
            ]
        );
    }
}
