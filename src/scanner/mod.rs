
use std::{iter::Peekable, str::Chars};

use crate::{
    Location,
    token::{Token, TokenError, TokenInstance, Tokens},
};

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
                if *c == '\n' {
                    self.location.newline();
                } else if c.is_whitespace() {
                    self.location.col += 1;
                } else {
                    break 'whitespace_loop;
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

    fn take_one_character(
        &mut self,
        chars: &mut Peekable<Chars<'_>>,
        instance: TokenInstance,
    ) -> Result<(), TokenError> {
        self.note_token(instance, 1);
        chars.next();
        Ok(())
    }

    fn take_two_characters(
        &mut self,
        chars: &mut Peekable<Chars<'_>>,
        instance: TokenInstance,
    ) -> Result<(), TokenError> {
        self.note_token(instance, 2);
        chars.next();
        chars.next();
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
        self.location.col += advance;
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

                    '(' => self.take_one_character(chars, TokenInstance::ParenLeft)?,

                    ')' => self.take_one_character(chars, TokenInstance::ParenRight)?,

                    '{' => self.take_one_character(chars, TokenInstance::BraceLeft)?,

                    '}' => self.take_one_character(chars, TokenInstance::BraceRight)?,

                    ',' => self.take_one_character(chars, TokenInstance::Comma)?,

                    '.' => self.take_one_character(chars, TokenInstance::Dot)?,

                    '-' => self.take_one_character(chars, TokenInstance::Minus)?,

                    '+' => self.take_one_character(chars, TokenInstance::Plus)?,

                    ';' => self.take_one_character(chars, TokenInstance::Semicolon)?,

                    '*' => self.take_one_character(chars, TokenInstance::Star)?,

                    '!' => {
                        if let Some('=') = chars.peek() {
                            self.take_two_characters(chars, TokenInstance::BangEqual)?
                        } else {
                            self.take_one_character(chars, TokenInstance::Bang)?
                        }
                    }

                    '=' => {
                        if let Some('=') = chars.peek() {
                            self.take_two_characters(chars, TokenInstance::EqualEqual)?
                        } else {
                            self.take_one_character(chars, TokenInstance::Equal)?
                        }
                    }

                    '<' => {
                        if let Some('=') = chars.peek() {
                            self.take_two_characters(chars, TokenInstance::LessEqual)?
                        } else {
                            self.take_one_character(chars, TokenInstance::Less)?
                        }
                    }

                    '>' => {
                        if let Some('=') = chars.peek() {
                            self.take_two_characters(chars, TokenInstance::GreaterEqual)?
                        } else {
                            self.take_one_character(chars, TokenInstance::Greater)?
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
                    location: Location { col: 0, line: 0 }
                },
                Token {
                    instance: TokenInstance::Number { literal: 0.23 },
                    location: Location { col: 2, line: 0 }
                },
                Token {
                    instance: TokenInstance::Number { literal: 1.23 },
                    location: Location { col: 2, line: 1 }
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
                    location: Location { col: 0, line: 0 }
                },
                Token {
                    instance: ParenLeft,
                    location: Location { col: 4, line: 0 }
                },
                Token {
                    instance: True,
                    location: Location { col: 5, line: 0 }
                },
                Token {
                    instance: And,
                    location: Location { col: 10, line: 0 }
                },
                Token {
                    instance: Identifier {
                        literal: "perhaps".to_string()
                    },
                    location: Location { col: 14, line: 0 }
                },
                Token {
                    instance: False,
                    location: Location { col: 22, line: 0 }
                },
                Token {
                    instance: ParenRight,
                    location: Location { col: 27, line: 0 }
                }
            ]
        );
    }
}
