use crate::ast::expression::{BinaryOp, Expression, UnaryOp};
use crate::ast::literal::Literal;
use crate::scanner::token::Token;
use crate::{ast::statement::Statement, scanner::token::TokenInstance};

use super::{ParseError, Parser};

impl Parser {
    pub fn token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn consume(&mut self) {
        self.index += 1
    }

    fn close_statement(&mut self) -> Result<(), ParseError> {
        match self.token() {
            Some(token) if token.instance == TokenInstance::Semicolon => {
                self.index += 1;
                Ok(())
            }

            _ => Err(ParseError::OpenStatement),
        }
    }
}

impl Parser {
    pub fn parse(&mut self) {
        self.statement();
    }

    fn statement(&mut self) -> Result<(), ParseError> {
        while let Some(token) = self.token() {
            use TokenInstance::*;

            match token.instance {
                Print => {
                    self.consume();
                    let expr = self.expression()?;
                    self.statements.push(Statement::Print { e: expr });
                    self.close_statement()?;
                }

                _ => todo!("{:?}", token.instance),
            }
        }

        Ok(())
    }

    pub fn expression(&mut self) -> Result<Expression, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.token() {
            match &token.instance {
                TokenInstance::EqualEqual => {
                    self.consume();
                    let right = self.comparison()?;
                    expr = Expression::Binary {
                        op: BinaryOp::Eq,
                        l: Box::new(expr),
                        r: Box::new(right),
                    }
                }

                TokenInstance::BangEqual => {
                    self.consume();
                    let right = self.comparison()?;
                    expr = Expression::Binary {
                        op: BinaryOp::Neq,
                        l: Box::new(expr),
                        r: Box::new(right),
                    };
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;

        while let Some(token) = self.token() {
            match &token.instance {
                TokenInstance::Greater => {
                    self.consume();
                    expr = Expression::Binary {
                        op: BinaryOp::Gt,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    };
                }

                TokenInstance::GreaterEqual => {
                    self.consume();
                    expr = Expression::Binary {
                        op: BinaryOp::Geq,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    }
                }

                TokenInstance::Less => {
                    self.consume();
                    expr = Expression::Binary {
                        op: BinaryOp::Lt,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    }
                }

                TokenInstance::LessEqual => {
                    self.consume();
                    expr = Expression::Binary {
                        op: BinaryOp::Leq,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    }
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;

        while let Some(token) = self.token() {
            match &token.instance {
                TokenInstance::Minus => {
                    self.consume();
                    expr = Expression::Binary {
                        op: BinaryOp::Minus,
                        l: Box::new(expr),
                        r: Box::new(self.term()?),
                    };
                }

                TokenInstance::Plus => {
                    self.consume();
                    expr = Expression::Binary {
                        op: BinaryOp::Plus,
                        l: Box::new(expr),
                        r: Box::new(self.term()?),
                    };
                }

                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;

        while let Some(token) = self.token() {
            match &token.instance {
                TokenInstance::Slash => {
                    self.consume();
                    let right = self.factor()?;
                    expr = Expression::Binary {
                        op: BinaryOp::Slash,
                        l: Box::new(expr),
                        r: Box::new(right),
                    };
                }

                TokenInstance::Star => {
                    self.consume();
                    let right = self.factor()?;
                    expr = Expression::Binary {
                        op: BinaryOp::Star,
                        l: Box::new(expr),
                        r: Box::new(right),
                    };
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
                let expr = match &token.instance {
                    TokenInstance::Bang => {
                        self.consume();
                        Expression::Unary {
                            op: UnaryOp::Bang,
                            e: Box::new(self.unary()?),
                        }
                    }
                    TokenInstance::Minus => {
                        self.consume();
                        Expression::Unary {
                            op: UnaryOp::Minus,
                            e: Box::new(self.unary()?),
                        }
                    }

                    _ => self.primary()?,
                };

                Ok(expr)
            }
        }
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        use crate::scanner::token::TokenInstance::*;
        match self.token() {
            None => return Err(ParseError::MissingToken),

            Some(token) => {
                let expr = match &token.instance {
                    Number { literal } => Expression::Literal {
                        l: Literal::from(*literal),
                    },

                    String { literal } => Expression::Literal {
                        l: Literal::from(literal.to_owned()),
                    },

                    True => Expression::Literal { l: Literal::True },

                    False => Expression::Literal { l: Literal::False },

                    Nil => Expression::Literal { l: Literal::Nil },

                    ParenLeft => {
                        self.consume();
                        let expr = self.expression()?;
                        if self.token().is_none_or(|token| !token.is(ParenRight)) {
                            return Err(ParseError::MismatchedParentheses);
                        }
                        expr
                    }

                    _ => {
                        return Err(ParseError::UnexpectedToken);
                    }
                };

                self.consume();
                Ok(expr)
            }
        }
    }
}

impl Parser {
    pub fn syncronise(&mut self) -> bool {
        while let Some(token) = self.token() {
            match &token.instance {
                TokenInstance::Semicolon => {
                    self.consume();
                    return true;
                }

                _ => {
                    self.consume();
                    continue;
                }
            }
        }

        false
    }
}
