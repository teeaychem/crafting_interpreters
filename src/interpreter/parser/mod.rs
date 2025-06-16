use crate::interpreter::{
    ast::statement::Statements,
    scanner::token::{Tkn, TknK},
};

use super::{
    TreeWalker,
    err::{Stumble, StumbleKind},
};

mod parse;

#[cfg(test)]
mod tests;

impl TreeWalker {
    pub fn statements(&self) -> &Statements {
        &self.statements
    }
}

impl TreeWalker {
    pub fn token(&self) -> Option<&Tkn> {
        self.tokens.get(self.token_index)
    }

    pub fn token_kind(&self) -> Option<&TknK> {
        match self.tokens.get(self.token_index) {
            Some(token) => Some(&token.kind),
            None => None,
        }
    }

    pub fn token_ahead(&self, ahead: usize) -> Option<&Tkn> {
        self.tokens.get(self.token_index + ahead)
    }

    pub fn token_kind_ahead(&self, ahead: usize) -> Option<&TknK> {
        match self.tokens.get(self.token_index + ahead) {
            Some(token) => Some(&token.kind),
            None => None,
        }
    }

    unsafe fn consume_unchecked(&mut self) {
        self.token_index += 1
    }

    fn check_token(&mut self, check: &TknK) -> Result<(), Stumble> {
        match self.token() {
            Some(t) if t.kind == *check => Ok(()),

            _ => {
                println!("Failed to find token {check:?}");

                Err(self.stumble_token(StumbleKind::MissingToken))
            }
        }
    }

    fn consume(&mut self, check: &TknK) -> Result<(), Stumble> {
        self.check_token(check)?;
        self.token_index += 1;
        Ok(())
    }

    fn close_statement(&mut self) -> Result<(), Stumble> {
        match self.token() {
            Some(token) if token.kind == TknK::Semicolon => {
                self.token_index += 1;
                Ok(())
            }

            _ => Err(self.stumble_token(StumbleKind::OpenStatement)),
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
