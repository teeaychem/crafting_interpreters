use crate::interpreter::{
    ast::statement::Statements,
    scanner::token::{Tkn, TknK},
};

use super::{
    TreeWalker,
    environment::{Env, EnvHandle},
    location::Location,
    scanner::token::Tkns,
};

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

impl TreeWalker {
    pub fn statements(&self) -> &Statements {
        &self.statements
    }
}

impl TreeWalker {
    pub fn token(&self) -> Option<&Tkn> {
        self.tokens.get(self.index)
    }

    pub fn token_kind(&self) -> Option<&TknK> {
        match self.tokens.get(self.index) {
            Some(token) => Some(&token.kind),
            None => None,
        }
    }

    pub fn token_ahead(&self, ahead: usize) -> Option<&Tkn> {
        self.tokens.get(self.index + ahead)
    }

    pub fn token_kind_ahead(&self, ahead: usize) -> Option<&TknK> {
        match self.tokens.get(self.index + ahead) {
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

impl TreeWalker {
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
