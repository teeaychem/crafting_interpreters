use crate::interpreter::{
    ast::statement::{Statement, Statements},
    scanner::{
        self, Scanner,
        token::{self, Tkn, TknK, Tkns},
    },
};

use super::environment::{Env, EnvHandle};

mod parse;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum ParseErr {
    CallArgLimit,
    ExpectedAssignment,
    ExpectedFound { expected: TknK, found: TknK },
    ForInitialiser,
    InvalidAsignee,
    MismatchedParentheses,
    MissingToken,
    OpenStatement,
    Todo,
    TokensExhausted,
    Unexpected { found: TknK },
}

#[derive(Debug)]
pub struct Parser {
    scanner: Scanner,
    statements: Statements,
    index: usize,
    parse_env: EnvHandle,
}

impl From<Scanner> for Parser {
    fn from(value: Scanner) -> Self {
        Self {
            index: 0,
            scanner: value,
            statements: Vec::default(),
            parse_env: Env::fresh_std_env(),
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            index: 0,
            scanner: Scanner::default(),
            statements: Statements::default(),
            parse_env: Env::fresh_std_env(),
        }
    }
}

impl Parser {
    pub fn scan<I: AsRef<str>>(&mut self, src: I) {
        self.scanner.scan(src);
    }

    pub fn take_scaner(&mut self, scanner: Scanner) {
        self.scanner.tokens = scanner.tokens
    }

    pub fn statements(&self) -> &Statements {
        &self.statements
    }

    pub fn push_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl Parser {
    pub fn token(&self) -> Option<&Tkn> {
        self.scanner.tokens.get(self.index)
    }

    pub fn token_kind(&self) -> Option<&TknK> {
        match self.scanner.tokens.get(self.index) {
            Some(token) => Some(&token.kind),
            None => None,
        }
    }

    pub fn token_ahead(&self, ahead: usize) -> Option<&Tkn> {
        self.scanner.tokens.get(self.index + ahead)
    }

    pub fn token_kind_ahead(&self, ahead: usize) -> Option<&TknK> {
        match self.scanner.tokens.get(self.index + ahead) {
            Some(token) => Some(&token.kind),
            None => None,
        }
    }

    unsafe fn consume_unchecked(&mut self) {
        self.index += 1
    }

    fn check_token(&mut self, check: &TknK) -> Result<(), ParseErr> {
        match self.token() {
            Some(t) if t.kind == *check => Ok(()),

            _ => {
                println!("Failed to find token {check:?}");
                Err(ParseErr::MissingToken)
            }
        }
    }

    fn consume(&mut self, check: &TknK) -> Result<(), ParseErr> {
        self.check_token(check)?;
        self.index += 1;
        Ok(())
    }

    fn close_statement(&mut self) -> Result<(), ParseErr> {
        match self.token() {
            Some(token) if token.kind == TknK::Semicolon => {
                self.index += 1;
                Ok(())
            }

            _ => Err(ParseErr::OpenStatement),
        }
    }
}

impl Parser {
    pub fn syncronise(&mut self) -> bool {
        println!("syncronising parser");
        while let Some(token) = self.token() {
            match &token.kind {
                TknK::Semicolon => {
                    self.consume(&TknK::Semicolon);
                    return true;
                }

                _ => {
                    unsafe { self.consume_unchecked() };
                    continue;
                }
            }
        }

        false
    }
}
