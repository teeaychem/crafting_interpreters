use std::{iter::Peekable, str::Chars};

use crate::interpreter::{
    location::Location,
    scanner::token::{Tkn, TknErr, TknK, Tkns},
};

pub mod token;

#[cfg(test)]
mod scanner_tests;

pub struct Scanner {
    pub location: Location,
    pub tokens: Tkns,
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

    fn take_string(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<(), TknErr> {
        chars.next();
        let mut literal = String::default();

        while let Some(d) = chars.peek() {
            match d {
                '"' => {
                    let length = literal.len() + 2;

                    self.note_token(
                        TknK::String {
                            literal: std::mem::take(&mut literal),
                        },
                        length,
                    );
                    chars.next();
                    break;
                }

                '\n' => return Err(TknErr::MultilineString),

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

    fn take_numeric(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<(), TknErr> {
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
            return Err(TknErr::TrailingDot);
        }

        self.note_token(
            TknK::Number {
                literal: number.parse().unwrap(),
            },
            number.len(),
        );

        Ok(())
    }

    fn take_alphabetic(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<(), TknErr> {
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
            "and" => TknK::And,

            "break" => TknK::Break,

            "class" => TknK::Class,

            "else" => TknK::Else,

            "false" => TknK::False,

            "for" => TknK::For,

            "fun" => TknK::Function,

            "if" => TknK::If,

            "loop" => TknK::Loop,

            "nil" => TknK::Nil,

            "or" => TknK::Or,

            "print" => TknK::Print,

            "return" => TknK::Return,

            "super" => TknK::Super,

            "this" => TknK::This,

            "true" => TknK::True,

            "var" => TknK::Var,

            "while" => TknK::While,

            non_keyword => TknK::Identifier {
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
        instance: TknK,
    ) -> Result<(), TknErr> {
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

    fn note_token(&mut self, instance: TknK, advance: usize) {
        self.tokens.push(Tkn {
            kind: instance,
            location: self.location,
        });
        self.location.advance_col(advance);
    }

    pub fn take_token(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<bool, TknErr> {
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
                            self.note_token(TknK::Slash, 1);
                        }
                    }

                    '(' => self.take_characters(chars, 1, TknK::ParenL)?,

                    ')' => self.take_characters(chars, 1, TknK::ParenR)?,

                    '{' => self.take_characters(chars, 1, TknK::BraceL)?,

                    '}' => self.take_characters(chars, 1, TknK::BraceR)?,

                    ',' => self.take_characters(chars, 1, TknK::Comma)?,

                    '.' => self.take_characters(chars, 1, TknK::Dot)?,

                    '-' => self.take_characters(chars, 1, TknK::Minus)?,

                    '+' => self.take_characters(chars, 1, TknK::Plus)?,

                    ';' => self.take_characters(chars, 1, TknK::Semicolon)?,

                    '*' => self.take_characters(chars, 1, TknK::Star)?,

                    '!' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TknK::BangEqual)?
                        } else {
                            self.take_characters(chars, 0, TknK::Bang)?
                        }
                    }

                    '=' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TknK::EqualEqual)?
                        } else {
                            self.take_characters(chars, 0, TknK::Equal)?
                        }
                    }

                    '<' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TknK::LessEqual)?
                        } else {
                            self.take_characters(chars, 0, TknK::Less)?
                        }
                    }

                    '>' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            self.take_characters(chars, 1, TknK::GreaterEqual)?
                        } else {
                            self.take_characters(chars, 0, TknK::Greater)?
                        }
                    }

                    numeric if numeric.is_numeric() => self.take_numeric(chars)?,

                    alphabetic if alphabetic.is_alphabetic() => self.take_alphabetic(chars)?,

                    unrecognised => {
                        return Err(TknErr::Unrecognised {
                            character: *unrecognised,
                        });
                    }
                }

                Ok(true)
            }
        }
    }
}
