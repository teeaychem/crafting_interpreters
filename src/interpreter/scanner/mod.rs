use std::{iter::Peekable, str::Chars};

use crate::interpreter::scanner::token::{Tkn, TknK};

use super::{
    TreeWalker,
    err::{Stumble, StumbleKind},
};

pub mod token;

#[cfg(test)]
mod scanner_tests;

impl TreeWalker {
    // Append `src` to the scanner and tokenize.
    pub fn scan<I: AsRef<str>>(&mut self, src: I) {
        let source_end = self.source.len();

        self.source.push_str(src.as_ref());

        let held_source = std::mem::take(&mut self.source);

        let mut chars = held_source[source_end..].chars().peekable();

        loop {
            match self.take_token(&mut chars) {
                Ok(true) => continue,

                Ok(false) => break,

                Err(e) => panic!("! Scanning failed with error: {e:?}"),
            }
        }

        while let Ok(true) = self.take_token(&mut chars) {}
    }

    // Store `token` and advance the current location by `advance`.
    fn store_token(&mut self, kind: TknK, advance: usize) {
        self.tokens.push(Tkn::new(kind, self.location));
        self.location.advance_col(advance);
    }

    // Take some token from `chars` and store the result.
    fn take_token(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<bool, Stumble> {
        self.eat_whitespace(chars);

        match chars.peek() {
            None => Ok(false),

            Some(c) => {
                match c {
                    '"' => {
                        let literal = self.get_string(chars)?;
                        let advance = literal.len() + 2;

                        self.store_token(TknK::String { literal }, advance);
                    }

                    '/' => {
                        chars.next();
                        if let Some('/') = chars.peek() {
                            self.eat_until('\n', chars);
                        } else {
                            self.store_token(TknK::Slash, 1);
                        }
                    }

                    '(' => {
                        chars.next();
                        self.store_token(TknK::ParenL, 1);
                    }

                    ')' => {
                        chars.next();
                        self.store_token(TknK::ParenR, 1);
                    }

                    '{' => {
                        chars.next();
                        self.store_token(TknK::BraceL, 1);
                    }

                    '}' => {
                        chars.next();
                        self.store_token(TknK::BraceR, 1);
                    }

                    ',' => {
                        chars.next();
                        self.store_token(TknK::Comma, 1);
                    }

                    '.' => {
                        chars.next();
                        self.store_token(TknK::Dot, 1);
                    }

                    '-' => {
                        chars.next();
                        self.store_token(TknK::Minus, 1);
                    }

                    '+' => {
                        chars.next();
                        self.store_token(TknK::Plus, 1);
                    }

                    ';' => {
                        chars.next();
                        self.store_token(TknK::Semicolon, 1);
                    }

                    '*' => {
                        chars.next();
                        self.store_token(TknK::Star, 1);
                    }

                    '!' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            chars.next();
                            self.store_token(TknK::BangEqual, 2);
                        } else {
                            self.store_token(TknK::Bang, 1);
                        }
                    }

                    '=' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            chars.next();
                            self.store_token(TknK::EqualEqual, 2);
                        } else {
                            self.store_token(TknK::Equal, 1);
                        }
                    }

                    '<' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            chars.next();
                            self.store_token(TknK::LessEqual, 2);
                        } else {
                            self.store_token(TknK::Less, 1);
                        }
                    }

                    '>' => {
                        chars.next();
                        if let Some('=') = chars.peek() {
                            chars.next();
                            self.store_token(TknK::GreaterEqual, 2);
                        } else {
                            self.store_token(TknK::Greater, 1);
                        }
                    }

                    numeric if numeric.is_numeric() => {
                        let (number, chars) = self.get_f64(chars)?;
                        self.store_token(TknK::Number { literal: number }, chars);
                    }

                    alphabetic if alphabetic.is_alphabetic() => {
                        let (token_kind, advance) = self.get_keyword_or_identifier(chars)?;
                        self.store_token(token_kind, advance);
                    }

                    unrecognised => {
                        return Err(self.stumble(StumbleKind::Unrecognised {
                            character: *unrecognised,
                        }));
                    }
                }

                Ok(true)
            }
        }
    }
}

impl TreeWalker {
    // Consume indefinate whitespace.
    fn eat_whitespace(&mut self, chars: &mut Peekable<Chars<'_>>) {
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

    // Consume tokens until a (closing) `"` is found and return the enclosed string.
    fn get_string(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<String, Stumble> {
        chars.next();
        let mut literal = String::default();

        while let Some(d) = chars.peek() {
            match d {
                '"' => {
                    chars.next();
                    break;
                }

                '\n' => return Err(self.stumble(StumbleKind::MultilineString)),

                _ => {
                    literal.push(*d);
                    chars.next();
                }
            }
        }

        Ok(literal)
    }

    // Eat until a `c` is found or all tokens have been consumed.
    fn eat_until(&mut self, c: char, chars: &mut Peekable<Chars<'_>>) {
        chars.next();
        while chars.peek().is_some_and(|d| *d != c) {
            chars.next();
        }
        self.location.newline();
    }

    // Consume numeric tokens until and f64 is identified.
    fn get_f64(&mut self, chars: &mut Peekable<Chars<'_>>) -> Result<(f64, usize), Stumble> {
        let mut number = String::default();

        while let Some(c) = chars.peek() {
            if c.is_numeric() || *c == '.' {
                number.push(*c);
                chars.next();
            } else {
                break;
            }
        }

        if let Some('.') = number.chars().last() {
            return Err(self.stumble(StumbleKind::TrailingDot));
        }

        Ok((number.parse().unwrap(), number.len()))
    }

    // Consume alphabetic tokens and return either a keyword or identifier.
    fn get_keyword_or_identifier(
        &mut self,
        chars: &mut Peekable<Chars<'_>>,
    ) -> Result<(TknK, usize), Stumble> {
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

        Ok((instance, alphabetic.len()))
    }
}
