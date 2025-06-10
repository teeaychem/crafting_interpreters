use crate::interpreter::ast::expression::{Expr, ExprB, OpOne, OpTwo};
use crate::interpreter::ast::identifier::Identifier;
use crate::interpreter::environment::EnvHandle;
use crate::interpreter::scanner::token::{self, Tkn};
use crate::interpreter::{ast::statement::Statement, scanner::token::TknKind};

use super::{ParseErr, Parser};

impl Parser {
    pub fn get_identifier(&self, expr: &Expr) -> Result<Identifier, ParseErr> {
        match expr {
            Expr::Identifier { id: i } => Ok(i.to_owned()),

            _ => Err(ParseErr::Todo),
        }
    }

    pub fn get_identifiers(&self, expr: &Vec<Expr>) -> Result<Vec<Identifier>, ParseErr> {
        let mut identifiers = Vec::default();

        for e in expr {
            identifiers.push(self.get_identifier(e)?);
        }

        Ok(identifiers)
    }
}

impl Parser {
    pub fn parse(&mut self, env: &EnvHandle) -> Result<(), ParseErr> {
        loop {
            match self.declaration(env) {
                Ok(stmt) => self.push_statement(stmt),

                Err(ParseErr::TokensExhausted) => break,

                Err(e) => panic!("{e:?}"),
            }
        }

        Ok(())
    }

    fn declaration(&mut self, env: &EnvHandle) -> Result<Statement, ParseErr> {
        if let Some(TknKind::Var) = self.token_kind() {
            self.consume_unchecked();

            let stmt = match self.expression(env)? {
                Expr::Assignment { id, e: assignment } => {
                    Statement::mk_declaration(*id, Some(*assignment))
                }

                Expr::Identifier { id } => Statement::mk_declaration(Expr::mk_identifier(id), None),

                _ => return Err(ParseErr::ExpectedAssignment),
            };

            self.close_statement();

            Ok(stmt)
        } else {
            self.statement(env)
        }
    }

    fn statement(&mut self, env: &EnvHandle) -> Result<Statement, ParseErr> {
        use TknKind::*;
        let stmt;

        let token = match self.token() {
            Some(t) => t,
            None => return Err(ParseErr::TokensExhausted),
        };

        match token.kind {
            Var => panic!("Higher precedence"),

            Print => {
                self.consume_unchecked();
                let expr = self.expression(env)?;
                self.close_statement()?;

                stmt = Statement::mk_print(expr);
            }

            BraceLeft => {
                self.consume_unchecked();

                let mut statements = Vec::default();

                while self.token_kind().is_some_and(|kind| *kind != BraceRight) {
                    statements.push(self.declaration(env)?);
                }

                self.consume_checked(&BraceRight);

                stmt = Statement::Block { statements };
            }

            If => {
                self.consume_unchecked();
                self.consume_checked(&ParenLeft);
                let expr = self.expression(env)?;
                self.consume_checked(&ParenRight);

                let case_if = self.declaration(env)?;
                let mut case_else = None;

                if let Some(t) = self.token() {
                    if t.kind == TknKind::Else {
                        self.consume_unchecked();
                        case_else = Some(self.declaration(env)?);
                    }
                }

                self.close_statement();

                stmt = Statement::mk_conditional(expr, case_if, case_else);
            }

            While => {
                self.consume_unchecked();
                self.consume_checked(&ParenLeft);
                let condition = self.expression(env)?;
                self.consume_checked(&ParenRight);

                let body = self.statement(env)?;

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

                let initialiser = self.declaration(env)?;
                match initialiser {
                    Statement::Declaration { .. } => loop_block.push(initialiser),

                    Statement::Empty => {}

                    _ => return Err(ParseErr::ForInitialiser),
                }

                let condition = match self.expression_delimited(env, TknKind::Semicolon)? {
                    Expr::Empty => Expr::mk_true(),
                    e => e,
                };
                self.consume_checked(&Semicolon);

                let increment = self.expression_delimited(env, TknKind::ParenRight)?;
                self.consume_checked(&ParenRight);

                let mut loop_statements = match self.statement(env)? {
                    Statement::Block { statements } => statements,

                    statement => vec![statement],
                };

                match increment {
                    Expr::Empty => {}

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

            Fun => {
                self.consume_checked(&Fun);

                let (id, params) = match self.expression(env)? {
                    Expr::Call { caller, args } => {
                        (self.get_identifier(&caller)?, self.get_identifiers(&args)?)
                    }

                    _ => {
                        return Err(ParseErr::Unexpected {
                            found: self.token().unwrap().kind.to_owned(),
                        });
                    }
                };

                let body = match self.statement(env)? {
                    Statement::Block { statements } => statements,

                    _ => panic!("! Block expected"),
                };

                stmt = Statement::mk_fun(id, params, body);
            }

            Semicolon => stmt = Statement::Empty,

            Return => {
                self.consume_checked(&Return);
                let rexpr = self.expression_delimited(env, Semicolon)?;
                println!("{rexpr}");
                self.consume_checked(&Semicolon);
                stmt = Statement::Return { expr: rexpr }
            }

            _ => match self.expression(env) {
                Err(_) => todo!("Statement {:?}", self.token()),

                Ok(expr) => {
                    self.close_statement()?;
                    stmt = Statement::mk_expression(expr);
                }
            },
        }

        Ok(stmt)
    }

    /// Returns an Expression on a successful parse, or an Expression::Empty on an unsuccesful parse due to an unexpected token of kind `delimiter`.
    pub fn expression_delimited(
        &mut self,
        env: &EnvHandle,
        delimiter: TknKind,
    ) -> Result<Expr, ParseErr> {
        match self.expression(env) {
            Ok(e) => Ok(e),

            Err(e) => match &e {
                ParseErr::Unexpected { found: delimiter } => Ok(Expr::Empty),

                _ => Err(e),
            },
        }
    }

    pub fn expression(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        self.assignment(env)
    }

    fn assignment(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        if let Some(TknKind::Identifier { id }) = self.token_kind() {
            if let Some(TknKind::Equal) = self.token_kind_ahead(1) {
                let id = Expr::mk_identifier(id.to_owned());

                self.consume_unchecked();
                self.consume_checked(&TknKind::Equal);
                let assignment = self.assignment(env)?;
                let expr = Expr::mk_assignment(id, assignment);

                return Ok(expr);
            }
        }

        self.logic_or(env)
    }

    fn logic_or(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        let mut expr = self.logic_and(env)?;

        while let Some(TknKind::Or) = self.token_kind() {
            self.consume_unchecked();
            let right = self.logic_and(env)?;
            expr = Expr::mk_or(expr, right);
        }

        Ok(expr)
    }

    fn logic_and(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        let mut expr = self.equality(env)?;

        while let Some(TknKind::And) = self.token_kind() {
            self.consume_unchecked();
            let right = self.equality(env)?;
            expr = Expr::mk_and(expr, right);
        }

        Ok(expr)
    }

    fn equality(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        let mut expr = self.comparison(env)?;

        while let Some(token) = self.token() {
            match &token.kind {
                TknKind::EqualEqual => {
                    self.consume_unchecked();
                    let right = self.comparison(env)?;
                    expr = Expr::mk_binary(OpTwo::Eq, expr, right)
                }

                TknKind::BangEqual => {
                    self.consume_unchecked();
                    let right = self.comparison(env)?;
                    expr = Expr::mk_binary(OpTwo::Neq, expr, right);
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        let mut expr = self.term(env)?;

        'comparison_match: while let Some(token) = self.token() {
            match &token.kind {
                TknKind::Greater => {
                    self.consume_unchecked();
                    expr = Expr::mk_binary(OpTwo::Gt, expr, self.comparison(env)?)
                }

                TknKind::GreaterEqual => {
                    self.consume_unchecked();
                    expr = Expr::mk_binary(OpTwo::Geq, expr, self.comparison(env)?)
                }

                TknKind::Less => {
                    self.consume_unchecked();
                    expr = Expr::mk_binary(OpTwo::Lt, expr, self.comparison(env)?)
                }

                TknKind::LessEqual => {
                    self.consume_unchecked();
                    expr = Expr::mk_binary(OpTwo::Leq, expr, self.comparison(env)?)
                }

                _ => break 'comparison_match,
            }
        }

        Ok(expr)
    }

    fn term(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        let mut expr = self.factor(env)?;

        while let Some(token) = self.token() {
            match &token.kind {
                TknKind::Minus => {
                    self.consume_unchecked();
                    expr = Expr::mk_binary(OpTwo::Minus, expr, self.term(env)?)
                }

                TknKind::Plus => {
                    self.consume_unchecked();
                    expr = Expr::mk_binary(OpTwo::Plus, expr, self.term(env)?)
                }

                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        let mut expr = self.unary(env)?;

        while let Some(token) = self.token() {
            match &token.kind {
                TknKind::Slash => {
                    self.consume_unchecked();
                    expr = Expr::mk_binary(OpTwo::Slash, expr, self.factor(env)?)
                }

                TknKind::Star => {
                    self.consume_unchecked();
                    expr = Expr::mk_binary(OpTwo::Star, expr, self.factor(env)?)
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        match self.token() {
            None => Err(ParseErr::MissingToken),

            Some(token) => {
                let expr = match &token.kind {
                    TknKind::Bang => {
                        self.consume_unchecked();
                        Expr::mk_unary(OpOne::Bang, self.unary(env)?)
                    }
                    TknKind::Minus => {
                        self.consume_unchecked();
                        Expr::mk_unary(OpOne::Minus, self.unary(env)?)
                    }

                    _ => self.call(env)?,
                };

                Ok(expr)
            }
        }
    }

    #[allow(clippy::while_let_loop)]
    fn call(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        let mut expr = self.primary(env)?;

        loop {
            match self.token_kind() {
                Some(TknKind::ParenLeft) => {
                    self.consume_checked(&TknKind::ParenLeft);
                    let mut args = Vec::default();
                    while self
                        .token_kind()
                        .is_some_and(|kind| *kind != TknKind::ParenRight)
                    {
                        args.push(self.expression(env)?);
                        if 255 <= args.len() {
                            return Err(ParseErr::CallArgLimit);
                        }

                        if let Some(TknKind::Comma) = self.token_kind() {
                            self.consume_checked(&TknKind::Comma);
                        }
                    }

                    self.consume_checked(&TknKind::ParenRight);

                    expr = Expr::mk_call(expr, args);
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn primary(&mut self, env: &EnvHandle) -> Result<Expr, ParseErr> {
        use crate::interpreter::scanner::token::TknKind::*;
        match self.token() {
            None => Err(ParseErr::MissingToken),

            Some(token) => {
                let expr = match &token.kind {
                    Number { literal } => Expr::mk_numeric(*literal),

                    String { literal } => Expr::mk_string(literal.to_owned()),

                    True => Expr::mk_true(),

                    False => Expr::mk_false(),

                    Nil => Expr::mk_nil(),

                    Identifier { id } => Expr::mk_identifier(id.to_owned()),

                    ParenLeft => {
                        self.consume_unchecked();
                        let expr = self.expression(env)?;
                        self.check_token(&ParenRight);

                        expr
                    }

                    _ => {
                        return Err(ParseErr::Unexpected {
                            found: token.kind.to_owned(),
                        });
                    }
                };

                self.consume_unchecked();
                Ok(expr)
            }
        }
    }
}
