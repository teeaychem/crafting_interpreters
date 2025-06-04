use crate::interpreter::ast::expression::{Expression, OpOne, OpTwo};
use crate::interpreter::ast::literal::Literal;
use crate::interpreter::scanner::token::{self, Token};
use crate::interpreter::{ast::statement::Statement, scanner::token::TokenKind};

use super::{ParseError, Parser};

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
    pub fn parse(&mut self) -> Result<(), ParseError> {
        loop {
            match self.declaration() {
                Ok(stmt) => self.add_statement(stmt),

                Err(ParseError::TokensExhausted) => break,

                Err(e) => panic!("{e:?}"),
            }
        }

        Ok(())
    }

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        if let Some(TokenKind::Var) = self.token_kind() {
            self.consume_unchecked();

            let stmt;

            match self.expression()? {
                Expression::Assignment { id, e: assignment } => {
                    stmt = Statement::mk_declaration(*id, Some(*assignment));
                }

                Expression::Identifier { id } => {
                    stmt = Statement::mk_declaration(Expression::mk_identifier(id), None);
                }

                _ => return Err(ParseError::ExpectedAssignment),
            };

            self.close_statement();

            Ok(stmt)
        } else {
            self.statement()
        }
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        use TokenKind::*;
        let stmt;

        let token = match self.token() {
            Some(t) => t,
            None => return Err(ParseError::TokensExhausted),
        };

        match token.kind {
            Print => {
                self.consume_unchecked();
                let expr = self.expression()?;
                stmt = Statement::mk_print(expr);
                self.close_statement()?;
            }

            Var => {
                panic!("Higher precedence")
            }

            BraceLeft => {
                self.consume_unchecked();

                let mut statements = Vec::default();

                while self.token_kind().is_some_and(|kind| *kind != BraceRight) {
                    statements.push(self.declaration()?);
                }

                self.consume_checked(&BraceRight);

                stmt = Statement::Block { statements };
            }

            If => {
                self.consume_unchecked();
                self.consume_checked(&ParenLeft);
                let expr = self.expression()?;
                self.consume_checked(&ParenRight);

                let case_if = self.declaration()?;
                let mut case_else = None;

                if let Some(t) = self.token() {
                    if t.kind == TokenKind::Else {
                        self.consume_unchecked();
                        case_else = Some(self.declaration()?);
                    }
                }

                stmt = Statement::mk_conditional(expr, case_if, case_else);

                self.close_statement();
            }

            While => {
                self.consume_unchecked();
                self.consume_checked(&ParenLeft);
                let condition = self.expression()?;
                self.consume_checked(&ParenRight);

                let body = self.statement()?;

                stmt = Statement::mk_while(condition, body);
            }

            For => {
                // Desugar the for to a while loop.
                // A little more specifically, to a block containig:
                // - The initialiser if present
                // - A while statement with the condition if present or a default `true` expression
                // - With the body of the while extended with the increment statement if present.

                self.consume_checked(&For);

                let mut loop_block = Vec::default();

                self.consume_checked(&ParenLeft);

                let initialiser = self.declaration()?;
                match initialiser {
                    Statement::Declaration { .. } => loop_block.push(initialiser),

                    Statement::Empty => {}

                    _ => return Err(ParseError::ForInitialiser),
                }

                let condition = match self.expression_delimited(TokenKind::Semicolon)? {
                    Expression::Empty => Expression::mk_true(),
                    e => e,
                };
                self.consume_checked(&Semicolon);

                let increment = self.expression_delimited(TokenKind::ParenRight)?;
                self.consume_checked(&ParenRight);

                let mut loop_statements = match self.statement()? {
                    Statement::Block { statements } => statements,

                    statement => vec![statement],
                };

                match increment {
                    Expression::Empty => {}

                    _ => loop_statements.push(Statement::mk_expression(increment)),
                }

                loop_block.push(Statement::mk_while(
                    condition,
                    Statement::Block {
                        statements: loop_statements,
                    },
                ));

                stmt = Statement::mk_block(loop_block);
            }

            Semicolon => stmt = Statement::Empty,

            _ => match self.expression() {
                Err(_) => todo!("Statment {:?}", self.token()),

                Ok(e) => {
                    stmt = Statement::mk_expression(e);
                    self.close_statement()?;
                }
            },
        }

        Ok(stmt)
    }

    /// Returns an Expression on a successful parse, or an Expression::Empty on an unsuccesful parse due to an unexpected token of kind `delimiter`.
    pub fn expression_delimited(&mut self, delimiter: TokenKind) -> Result<Expression, ParseError> {
        match self.expression() {
            Ok(e) => Ok(e),

            Err(e) => match &e {
                ParseError::UnexpectedToken {
                    token: Token {
                        kind: exception, ..
                    },
                } => Ok(Expression::Empty),

                _ => Err(e),
            },
        }
    }

    pub fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        if let Some(TokenKind::Identifier { id }) = self.token_kind() {
            if let Some(TokenKind::Equal) = self.token_kind_ahead(1) {
                let id = Expression::mk_identifier(id.to_owned());

                self.consume_unchecked();
                self.consume_checked(&TokenKind::Equal);
                let assignment = self.assignment()?;
                let expr = Expression::mk_assignment(id, assignment);

                return Ok(expr);
            }
        }

        self.logic_or()
    }

    fn logic_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.logic_and()?;

        while let Some(TokenKind::Or) = self.token_kind() {
            self.consume_unchecked();
            let right = self.logic_and()?;
            expr = Expression::mk_or(expr, right);
        }

        Ok(expr)
    }

    fn logic_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        while let Some(TokenKind::And) = self.token_kind() {
            self.consume_unchecked();
            let right = self.equality()?;
            expr = Expression::mk_and(expr, right);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.token() {
            match &token.kind {
                TokenKind::EqualEqual => {
                    self.consume_unchecked();
                    let right = self.comparison()?;
                    expr = Expression::mk_binary(OpTwo::Eq, expr, right)
                }

                TokenKind::BangEqual => {
                    self.consume_unchecked();
                    let right = self.comparison()?;
                    expr = Expression::mk_binary(OpTwo::Neq, expr, right);
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;

        'comparison_match: while let Some(token) = self.token() {
            match &token.kind {
                TokenKind::Greater => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Gt, expr, self.comparison()?)
                }

                TokenKind::GreaterEqual => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Geq, expr, self.comparison()?)
                }

                TokenKind::Less => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Lt, expr, self.comparison()?)
                }

                TokenKind::LessEqual => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Leq, expr, self.comparison()?)
                }

                _ => break 'comparison_match,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;

        while let Some(token) = self.token() {
            match &token.kind {
                TokenKind::Minus => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Minus, expr, self.term()?)
                }

                TokenKind::Plus => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Plus, expr, self.term()?)
                }

                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;

        while let Some(token) = self.token() {
            match &token.kind {
                TokenKind::Slash => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Slash, expr, self.factor()?)
                }

                TokenKind::Star => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Star, expr, self.factor()?)
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        match self.token() {
            None => Err(ParseError::MissingToken),

            Some(token) => {
                let expr = match &token.kind {
                    TokenKind::Bang => {
                        self.consume_unchecked();
                        Expression::mk_unary(OpOne::Bang, self.unary()?)
                    }
                    TokenKind::Minus => {
                        self.consume_unchecked();
                        Expression::mk_unary(OpOne::Minus, self.unary()?)
                    }

                    _ => self.call()?,
                };

                Ok(expr)
            }
        }
    }

    fn call(&mut self) -> Result<Expression, ParseError> {
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        use crate::interpreter::scanner::token::TokenKind::*;
        match self.token() {
            None => Err(ParseError::MissingToken),

            Some(token) => {
                let expr = match &token.kind {
                    Number { literal } => Expression::mk_literal(Literal::from(*literal)),

                    String { literal } => Expression::mk_literal(Literal::from(literal.to_owned())),

                    True => Expression::mk_literal(Literal::True),

                    False => Expression::mk_literal(Literal::False),

                    Nil => Expression::mk_literal(Literal::Nil),

                    Identifier { id } => Expression::mk_identifier(id.to_owned()),

                    ParenLeft => {
                        self.consume_unchecked();
                        let expr = self.expression()?;
                        self.check_token(&ParenRight);

                        expr
                    }

                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            token: token.clone(),
                        });
                    }
                };

                self.consume_unchecked();
                Ok(expr)
            }
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
