use crate::interpreter::{
    ast::statement::{Statement, Statements},
    scanner::{
        self,
        token::{self, Token, TokenKind, Tokens},
        Scanner,
    },
};

mod parse;
pub mod value;

#[derive(Debug)]
pub enum ParseError {
    MismatchedParentheses,
    UnexpectedToken { token: Token },
    MissingToken,
    OpenStatement,
    InvalidAsignee,
    ExpectedAssignment,
    TokensExhausted,
    ForInitialiser
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

