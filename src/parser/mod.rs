use crate::{
    expression::{Expression, UnaryOp},
    scanner::Scanner,
    token::{Token, TokenInstance, Tokens},
};

#[derive(Debug)]
pub enum ParseError {
    MismatchedParentheses,
    UnexpectedToken,
    MissingToken,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Tokens,
    index: usize,
}

impl From<Scanner> for Parser {
    fn from(value: Scanner) -> Self {
        Self {
            tokens: value.tokens,
            index: 0,
        }
    }
}

impl Parser {
    fn token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn consume(&mut self) {
        self.index += 1
    }
}

impl Parser {
    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(token) = self.token() {
            use crate::expression::Expression::*;

            match &token.instance {
                TokenInstance::EqualEqual => {
                    self.consume();
                    let right = self.comparison()?;
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Eq,
                        l: Box::new(expr),
                        r: Box::new(right),
                    }
                }

                TokenInstance::BangEqual => {
                    self.consume();
                    let right = self.comparison()?;
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Neq,
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
            use crate::expression::Expression::*;

            match &token.instance {
                TokenInstance::Greater => {
                    self.consume();
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Gt,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    };
                }

                TokenInstance::GreaterEqual => {
                    self.consume();
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Geq,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    }
                }

                TokenInstance::Less => {
                    self.consume();
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Lt,
                        l: Box::new(expr),
                        r: Box::new(self.comparison()?),
                    }
                }

                TokenInstance::LessEqual => {
                    self.consume();
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Leq,
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
            use crate::expression::Expression::*;

            match &token.instance {
                TokenInstance::Minus => {
                    self.consume();
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Minus,
                        l: Box::new(expr),
                        r: Box::new(self.term()?),
                    };
                }

                TokenInstance::Plus => {
                    self.consume();
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Plus,
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
            use crate::expression::Expression::*;

            match &token.instance {
                TokenInstance::Slash => {
                    self.consume();
                    let right = self.factor()?;
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Slash,
                        l: Box::new(expr),
                        r: Box::new(right),
                    };
                }

                TokenInstance::Star => {
                    self.consume();
                    let right = self.factor()?;
                    expr = Expression::Binary {
                        op: crate::expression::BinaryOp::Star,
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
        use crate::token::TokenInstance::*;
        match self.token() {
            None => return Err(ParseError::MissingToken),

            Some(token) => {
                use crate::expression::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut scanner = Scanner::default();
        let input = "! true == false";

        scanner.scan(input);

        let mut parser = Parser::from(scanner);
        let expr = parser.expression();

        match expr {
            Ok(expr) => assert_eq!(format!("{expr}"), "(== (! true) false)"),

            Err(_) => panic!("Failed to parse {input}"),
        }
    }

    #[test]
    fn arithmetic() {
        let mut scanner = Scanner::default();
        let input = "4 / 3 * - 2";

        scanner.scan(input);

        let mut parser = Parser::from(scanner);
        let expr = parser.expression();

        match expr {
            Ok(expr) => assert_eq!(format!("{expr}"), "(/ 4 (* 3 (- 2)))"),

            Err(_) => panic!("Failed to parse {input}"),
        }
    }

    #[test]
    fn sync() {
        let mut scanner = Scanner::default();
        let input = "4 / ; + 2 2; 2 + 2";

        scanner.scan(input);

        let mut parser = Parser::from(scanner);
        let expr = parser.expression();

        match expr {
            Ok(expr) => panic!("Expected parse error"),

            Err(_) => loop {
                match parser.expression() {
                    Ok(expr) => {
                        assert_eq!(format!("{expr}"), "(+ 2 2)");
                        break;
                    }
                    Err(_) => {
                        if parser.token().is_none() {
                            panic!("Failed to sync before EOF");
                        }

                        parser.syncronise();
                    }
                }
            },
        }
    }
}
