use crate::interpreter::{
    ast::statement::{Statement, Statements},
    scanner::{
        self, Scanner,
        token::{self, Tkn, TknK, Tkns},
    },
};

mod parse;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum ParseErr {
    CallArgLimit,
    ExpectedAssignment,
    ForInitialiser,
    InvalidAsignee,
    MismatchedParentheses,
    MissingToken,
    OpenStatement,
    TokensExhausted,
    ExpectedFound { expected: TknK, found: TknK },
    Unexpected { found: TknK },
    Todo,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Tkns,
    statements: Statements,
    index: usize,
}

impl From<Scanner> for Parser {
    fn from(value: Scanner) -> Self {
        Self {
            tokens: value.tokens,
            statements: Vec::default(),
            index: 0,
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            tokens: Tkns::default(),
            statements: Statements::default(),
            index: 0,
        }
    }
}

impl Parser {
    pub fn take_scaner(&mut self, scanner: Scanner) {
        self.tokens = scanner.tokens
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
