use crate::interpreter::{
    ast::statement::{Statement, Statements},
    scanner::{
        self, Scanner,
        token::{self, Token, TokenKind, Tokens},
    },
};

mod parse;
pub mod value;

#[derive(Debug)]
pub enum ParseError {
    CallArgLimit,
    ExpectedAssignment,
    ForInitialiser,
    InvalidAsignee,
    MismatchedParentheses,
    MissingToken,
    OpenStatement,
    TokensExhausted,
    UnexpectedToken { token: Token },
}

#[derive(Debug)]
pub struct Parser {
    tokens: Tokens,
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
            tokens: Tokens::default(),
            statements: Statements::default(),
            index: 0,
        }
    }
}

impl Parser {
    pub fn consume_scanner(&mut self, scanner: Scanner) {
        self.tokens = scanner.tokens
    }

    pub fn statements(&self) -> &Statements {
        &self.statements
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

impl Parser {
    pub fn token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn token_kind(&self) -> Option<&TokenKind> {
        match self.tokens.get(self.index) {
            Some(token) => Some(&token.kind),
            None => None,
        }
    }

    pub fn token_ahead(&self, ahead: usize) -> Option<&Token> {
        self.tokens.get(self.index + ahead)
    }

    pub fn token_kind_ahead(&self, ahead: usize) -> Option<&TokenKind> {
        match self.tokens.get(self.index + ahead) {
            Some(token) => Some(&token.kind),
            None => None,
        }
    }

    fn consume_unchecked(&mut self) {
        self.index += 1
    }

    fn check_token(&mut self, check: &TokenKind) -> Result<(), ParseError> {
        match self.token() {
            Some(t) if t.kind == *check => Ok(()),

            _ => {
                println!("Failed to find token {check:?}");
                Err(ParseError::MissingToken)
            }
        }
    }

    fn consume_checked(&mut self, check: &TokenKind) -> Result<(), ParseError> {
        self.check_token(check)?;
        self.index += 1;
        Ok(())
    }

    fn close_statement(&mut self) -> Result<(), ParseError> {
        match self.token() {
            Some(token) if token.kind == TokenKind::Semicolon => {
                self.index += 1;
                Ok(())
            }

            _ => Err(ParseError::OpenStatement),
        }
    }
}

impl Parser {
    pub fn syncronise(&mut self) -> bool {
        println!("syncronising parser");
        while let Some(token) = self.token() {
            match &token.kind {
                TokenKind::Semicolon => {
                    self.consume_unchecked();
                    return true;
                }

                _ => {
                    self.consume_unchecked();
                    continue;
                }
            }
        }

        false
    }
}
