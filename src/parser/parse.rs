use crate::ast::expression::{Expression, OpB, OpU};
use crate::ast::literal::Literal;
use crate::scanner::token::{self, Token};
use crate::{ast::statement::Statement, scanner::token::TokenInstance};

use super::{ParseError, Parser};

impl Parser {
    pub fn token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn consume(&mut self) {
        self.index += 1
    }

    fn consume_specific(&mut self, instance: TokenInstance) -> Result<(), ParseError> {
        if self.token().is_some_and(|t| t.instance == instance) {
            self.index += 1;
            Ok(())
        } else {
            println!("Failed to consume {instance:?}");
            Err(ParseError::MissingToken)
        }
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
    pub fn parse(&mut self) -> Result<(), ParseError> {
        self.statement()
    }

    fn statement(&mut self) -> Result<(), ParseError> {
        while let Some(token) = self.token() {
            use TokenInstance::*;
            dbg!(&token);

            match token.instance {
                Print => {
                    self.consume();
                    let expr = self.expression()?;
                    self.add_statement(Statement::Print { e: expr });
                    self.close_statement()?;
                }

                Var => {
                    self.consume();

                    match self.expression()? {
                        Expression::Assignment { name, assignment } => {
                            self.add_statement(Statement::Declaration {
                                id: *name,
                                assignment: *assignment,
                            });
                        }

                        _ => return Err(ParseError::ExpectedAssignment),
                    };

                    self.close_statement()?;
                }

                _ => match self.expression() {
                    Err(_) => todo!("Statment todo"),

                    Ok(e) => {
                        self.add_statement(Statement::Expression { e });
                        self.close_statement()?;
                    }
                },
            }
        }

        Ok(())
    }

    pub fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        if let Some(token) = self.token() {
            match &token.instance {
                TokenInstance::Equal => {
                    self.consume();
                    let assignment = self.assignment()?;
                    expr = Expression::Assignment {
                        name: Box::new(expr),
                        assignment: Box::new(assignment),
                    };
                }

                _ => {}
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.token() {
            match &token.instance {
                TokenInstance::EqualEqual => {
                    self.consume();
                    let right = self.comparison()?;
                    expr = Expression::Binary {
                        op: OpB::Eq,
                        l: Box::new(expr),
                        r: Box::new(right),
                    }
                }

                TokenInstance::BangEqual => {
                    self.consume();
                    let right = self.comparison()?;
                    expr = Expression::Binary {
                        op: OpB::Neq,
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
                        op: OpB::Gt,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    };
                }

                TokenInstance::GreaterEqual => {
                    self.consume();
                    expr = Expression::Binary {
                        op: OpB::Geq,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    }
                }

                TokenInstance::Less => {
                    self.consume();
                    expr = Expression::Binary {
                        op: OpB::Lt,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    }
                }

                TokenInstance::LessEqual => {
                    self.consume();
                    expr = Expression::Binary {
                        op: OpB::Leq,
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
                        op: OpB::Minus,
                        l: Box::new(expr),
                        r: Box::new(self.term()?),
                    };
                }

                TokenInstance::Plus => {
                    self.consume();
                    expr = Expression::Binary {
                        op: OpB::Plus,
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
                        op: OpB::Slash,
                        l: Box::new(expr),
                        r: Box::new(right),
                    };
                }

                TokenInstance::Star => {
                    self.consume();
                    let right = self.factor()?;
                    expr = Expression::Binary {
                        op: OpB::Star,
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
                            op: OpU::Bang,
                            e: Box::new(self.unary()?),
                        }
                    }
                    TokenInstance::Minus => {
                        self.consume();
                        Expression::Unary {
                            op: OpU::Minus,
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

                    Identifier { literal } => Expression::Identifier {
                        l: Literal::from(literal.to_owned()),
                    },

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
        println!("syncronising parser");
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
