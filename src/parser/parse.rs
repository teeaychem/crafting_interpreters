use crate::ast::expression::{Expression, OpB, OpU};
use crate::ast::literal::Literal;
use crate::scanner::token::{self, Token};
use crate::{ast::statement::Statement, scanner::token::TokenKind};

use super::{ParseError, Parser};

impl Parser {
    pub fn token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
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
            match self.statement() {
                Ok(stmt) => self.add_statement(stmt),

                Err(ParseError::TokensExhausted) => break,

                Err(e) => panic!("{e:?}"),
            }
        }

        Ok(())
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
                stmt = Statement::print(expr);
                self.close_statement()?;
            }

            Var => {
                self.consume_unchecked();

                match self.expression()? {
                    Expression::Assignment { id, e: assignment } => {
                        stmt = Statement::declaration(*id, Some(*assignment));
                    }

                    Expression::Identifier { id } => {
                        stmt = Statement::declaration(Expression::identifier(id), None);
                    }

                    _ => return Err(ParseError::ExpectedAssignment),
                };

                self.close_statement()?;
            }

            BraceLeft => {
                stmt = Statement::BlockEnter;
                self.consume_unchecked();
            }

            BraceRight => {
                stmt = Statement::BlockExit;
                self.consume_unchecked();
            }

            If => {
                self.consume_unchecked();
                self.consume_checked(&ParenLeft);
                let expr = self.expression()?;
                self.consume_checked(&ParenRight);
                let yes = self.statement()?;
                let mut no = None;

                match self.token() {
                    Some(t) => {
                        if t.kind == TokenKind::Else {
                            self.consume_unchecked();
                            no = Some(Box::new(self.statement()?));
                        }
                    }

                    Some(_) | None => {}
                }

                stmt = Statement::Conditional {
                    condition: expr,
                    yes: Box::new(yes),
                    no,
                };

                self.close_statement();
            }

            _ => match self.expression() {
                Err(_) => todo!("Statment todo"),

                Ok(e) => {
                    stmt = Statement::Expression { e };
                    self.close_statement()?;
                }
            },
        }

        Ok(stmt)
    }

    pub fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        if let Some(token) = self.token() {
            match &token.kind {
                TokenKind::Equal => {
                    self.consume_unchecked();
                    let assignment = self.assignment()?;
                    expr = Expression::assignment(expr, assignment)
                }

                _ => {}
            }
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
                    expr = Expression::binary(OpB::Eq, expr, right)
                }

                TokenKind::BangEqual => {
                    self.consume_unchecked();
                    let right = self.comparison()?;
                    expr = Expression::binary(OpB::Neq, expr, right);
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;

        while let Some(token) = self.token() {
            match &token.kind {
                TokenKind::Greater => {
                    self.consume_unchecked();
                    expr = Expression::binary(OpB::Gt, expr, self.comparison()?)
                }

                TokenKind::GreaterEqual => {
                    self.consume_unchecked();
                    expr = Expression::binary(OpB::Geq, expr, self.comparison()?)
                }

                TokenKind::Less => {
                    self.consume_unchecked();
                    expr = Expression::binary(OpB::Lt, expr, self.comparison()?)
                }

                TokenKind::LessEqual => {
                    self.consume_unchecked();
                    expr = Expression::binary(OpB::Leq, expr, self.comparison()?)
                }

                _ => break,
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
                    expr = Expression::binary(OpB::Minus, expr, self.term()?)
                }

                TokenKind::Plus => {
                    self.consume_unchecked();
                    expr = Expression::binary(OpB::Plus, expr, self.term()?)
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
                    expr = Expression::binary(OpB::Slash, expr, self.factor()?)
                }

                TokenKind::Star => {
                    self.consume_unchecked();
                    expr = Expression::binary(OpB::Star, expr, self.factor()?)
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        match self.token() {
            None => return Err(ParseError::MissingToken),

            Some(token) => {
                let expr = match &token.kind {
                    TokenKind::Bang => {
                        self.consume_unchecked();
                        Expression::unary(OpU::Bang, self.unary()?)
                    }
                    TokenKind::Minus => {
                        self.consume_unchecked();
                        Expression::unary(OpU::Minus, self.unary()?)
                    }

                    _ => self.primary()?,
                };

                Ok(expr)
            }
        }
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        use crate::scanner::token::TokenKind::*;
        match self.token() {
            None => return Err(ParseError::MissingToken),

            Some(token) => {
                let expr = match &token.kind {
                    Number { literal } => Expression::literal(Literal::from(*literal)),

                    String { literal } => Expression::literal(Literal::from(literal.to_owned())),

                    True => Expression::literal(Literal::True),

                    False => Expression::literal(Literal::False),

                    Nil => Expression::literal(Literal::Nil),

                    Identifier { id } => Expression::identifier(Literal::from(id.to_owned())),

                    ParenLeft => {
                        self.consume_unchecked();
                        let expr = self.expression()?;
                        self.check_token(&ParenRight);

                        expr
                    }

                    _ => {
                        return Err(ParseError::UnexpectedToken);
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
