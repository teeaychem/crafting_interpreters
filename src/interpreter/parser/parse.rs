use crate::interpreter::ast::expression::{Expression, OpOne, OpTwo};
use crate::interpreter::ast::literal::Literal;
use crate::interpreter::scanner::token::{self, Tkn};
use crate::interpreter::{ast::statement::Statement, scanner::token::TknKind};

use super::{ParseError, Parser};

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
        if let Some(TknKind::Var) = self.token_kind() {
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
        use TknKind::*;
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
                    if t.kind == TknKind::Else {
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

                let condition = match self.expression_delimited(TknKind::Semicolon)? {
                    Expression::Empty => Expression::mk_true(),
                    e => e,
                };
                self.consume_checked(&Semicolon);

                let increment = self.expression_delimited(TknKind::ParenRight)?;
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
                Err(_) => todo!("Statement {:?}", self.token()),

                Ok(expr) => {
                    stmt = Statement::mk_expression(expr);
                    self.close_statement()?;
                }
            },
        }

        Ok(stmt)
    }

    /// Returns an Expression on a successful parse, or an Expression::Empty on an unsuccesful parse due to an unexpected token of kind `delimiter`.
    pub fn expression_delimited(&mut self, delimiter: TknKind) -> Result<Expression, ParseError> {
        match self.expression() {
            Ok(e) => Ok(e),

            Err(e) => match &e {
                ParseError::UnexpectedToken {
                    token: Tkn {
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
        if let Some(TknKind::Identifier { id }) = self.token_kind() {
            if let Some(TknKind::Equal) = self.token_kind_ahead(1) {
                let id = Expression::mk_identifier(id.to_owned());

                self.consume_unchecked();
                self.consume_checked(&TknKind::Equal);
                let assignment = self.assignment()?;
                let expr = Expression::mk_assignment(id, assignment);

                return Ok(expr);
            }
        }

        self.logic_or()
    }

    fn logic_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.logic_and()?;

        while let Some(TknKind::Or) = self.token_kind() {
            self.consume_unchecked();
            let right = self.logic_and()?;
            expr = Expression::mk_or(expr, right);
        }

        Ok(expr)
    }

    fn logic_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        while let Some(TknKind::And) = self.token_kind() {
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
                TknKind::EqualEqual => {
                    self.consume_unchecked();
                    let right = self.comparison()?;
                    expr = Expression::mk_binary(OpTwo::Eq, expr, right)
                }

                TknKind::BangEqual => {
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
                TknKind::Greater => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Gt, expr, self.comparison()?)
                }

                TknKind::GreaterEqual => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Geq, expr, self.comparison()?)
                }

                TknKind::Less => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Lt, expr, self.comparison()?)
                }

                TknKind::LessEqual => {
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
                TknKind::Minus => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Minus, expr, self.term()?)
                }

                TknKind::Plus => {
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
                TknKind::Slash => {
                    self.consume_unchecked();
                    expr = Expression::mk_binary(OpTwo::Slash, expr, self.factor()?)
                }

                TknKind::Star => {
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
                    TknKind::Bang => {
                        self.consume_unchecked();
                        Expression::mk_unary(OpOne::Bang, self.unary()?)
                    }
                    TknKind::Minus => {
                        self.consume_unchecked();
                        Expression::mk_unary(OpOne::Minus, self.unary()?)
                    }

                    _ => self.call()?,
                };

                Ok(expr)
            }
        }
    }

    #[allow(clippy::while_let_loop)]
    fn call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.primary()?;

        loop {
            match self.token_kind() {
                Some(TknKind::ParenLeft) => {
                    self.consume_checked(&TknKind::ParenLeft);
                    let mut args = Vec::default();
                    while self
                        .token_kind()
                        .is_some_and(|kind| *kind != TknKind::ParenRight)
                    {
                        args.push(self.expression()?);
                        if 255 <= args.len() {
                            return Err(ParseError::CallArgLimit);
                        }

                        if let Some(TknKind::Comma) = self.token_kind() {
                            self.consume_checked(&TknKind::Comma);
                        }
                    }

                    self.consume_checked(&TknKind::ParenRight);

                    expr = Expression::mk_call(expr, args);
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        use crate::interpreter::scanner::token::TknKind::*;
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
