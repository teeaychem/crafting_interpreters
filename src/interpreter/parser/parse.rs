use crate::interpreter::{
    TreeWalker,
    ast::{
        expression::{Expr, ExprB, OpOne, OpTwo},
        identifier::Identifier,
        statement::{Statement, Statements},
    },
    environment::{Env, EnvHandle},
    err::{Stumble, StumbleKind},
    scanner::token::TknK,
};

impl TreeWalker {
    pub fn to_identifier(&self, expr: Expr) -> Result<Identifier, Stumble> {
        match expr {
            Expr::Identifier { id: i } => Ok(i),

            _ => {
                panic!("! Identifier expected");
                // Err(Stumble::Todo)
            }
        }
    }

    pub fn to_identifiers(&self, expr: Vec<Expr>) -> Result<Vec<Identifier>, Stumble> {
        let mut identifiers = Vec::default();

        for e in expr {
            identifiers.push(self.to_identifier(e)?);
        }

        Ok(identifiers)
    }
}

impl TreeWalker {
    pub fn parse(&mut self) -> Result<(), Stumble> {
        let env = std::mem::take(&mut self.parse_env);

        loop {
            match self.declaration(&env) {
                Ok(stmt) => self.statements.push(stmt),

                Err(e) if *e.kind() == StumbleKind::TokensExhausted => break,

                Err(e) => panic!("{e:?}"),
            }
        }

        std::mem::replace(&mut self.parse_env, env);

        Ok(())
    }

    fn declaration(&mut self, env: &EnvHandle) -> Result<Statement, Stumble> {
        if let Some(TknK::Var) = self.token_kind() {
            self.consume(&TknK::Var);

            let primary_expr = self.primary(env)?;
            let d_id = self.to_identifier(primary_expr)?;
            let d_val = match self.token_kind() {
                Some(&TknK::Equal) => {
                    self.consume(&TknK::Equal);
                    self.expression(env)?
                }

                Some(_) => Expr::Basic(ExprB::Nil),

                None => panic!("! Unexpected EOF"),
            };

            env.borrow_mut().insert(d_id.name(), ExprB::Nil);

            self.close_statement();

            Ok(Statement::mk_declaration(d_id, d_val))
        } else {
            self.statement(env)
        }
    }

    fn block_statements(&mut self, block_env: &EnvHandle) -> Result<Statements, Stumble> {
        self.consume(&TknK::BraceL);

        let mut statements = Vec::default();

        while self.token_kind().is_some_and(|kind| kind != &TknK::BraceR) {
            statements.push(self.declaration(block_env)?);
        }

        self.consume(&TknK::BraceR);

        Ok(statements)
    }

    fn statement(&mut self, env: &EnvHandle) -> Result<Statement, Stumble> {
        let stmt;

        let token = match self.token() {
            Some(tkn) => tkn,

            None => return Err(self.stumble(StumbleKind::TokensExhausted)),
        };

        match token.kind {
            TknK::Var => panic!("Higher precedence"),

            TknK::Print => {
                self.consume(&TknK::Print);
                let expr = self.expression(env)?;
                self.close_statement()?;

                stmt = Statement::mk_print(expr);
            }

            TknK::BraceL => {
                let block_env = Env::narrow(env.clone());

                stmt = Statement::Block {
                    statements: self.block_statements(&block_env)?,
                };
            }

            TknK::If => {
                self.consume(&TknK::If);

                // TODO: Make parens optional, as they're purely decorative here.
                self.consume(&TknK::ParenL);

                let expr = self.expression(env)?;

                self.consume(&TknK::ParenR);

                let case_if = self.declaration(env)?;

                let mut case_else = None;

                if let Some(t) = self.token() {
                    if t.kind == TknK::Else {
                        self.consume(&TknK::Else);
                        case_else = Some(self.declaration(env)?);
                    }
                }

                self.close_statement();

                stmt = Statement::mk_conditional(expr, case_if, case_else);
            }

            TknK::Loop => {
                self.consume(&TknK::Loop);

                self.consume(&TknK::BraceL);

                let mut statements = Vec::default();

                let loop_env = Env::narrow(env.clone());

                while self.token_kind().is_some_and(|kind| kind != &TknK::BraceR) {
                    statements.push(self.declaration(&loop_env)?);
                }

                self.consume(&TknK::BraceR);

                stmt = Statement::mk_loop(statements);
            }

            TknK::While => {
                self.consume(&TknK::While);
                let loop_env = Env::narrow(env.clone());

                // TODO: Cosmetic parens
                self.consume(&TknK::ParenL);
                let condition = self.expression(&loop_env)?;
                self.consume(&TknK::ParenR);

                let statements = self.block_statements(&loop_env)?;

                stmt = Statement::mk_while(condition, statements);
            }

            TknK::For => {
                // Desugar the for to a while loop.
                // A little more specifically, to a block containig:
                // - The initialiser if present
                // - A while statement with the condition if present or a default `true` expression
                // - With the body of the while extended with the increment statement if present.

                self.consume(&TknK::For);

                let for_env = Env::narrow(env.clone());
                let while_env = Env::narrow(for_env.clone());

                let mut loop_block = Vec::default();

                self.consume(&TknK::ParenL);

                let initialiser = self.declaration(&for_env)?;
                match initialiser {
                    Statement::Declaration { .. } => loop_block.push(initialiser),

                    Statement::Empty => {}

                    _ => return Err(self.stumble(StumbleKind::ForInitialiser)),
                }

                let condition = match self.expression_delimited(&while_env, &TknK::Semicolon)? {
                    Expr::Empty => Expr::mk_true(),
                    e => e,
                };
                self.consume(&TknK::Semicolon);

                let increment = self.expression_delimited(&while_env, &TknK::ParenR)?;

                self.consume(&TknK::ParenR);

                let mut statements = self.block_statements(&while_env)?;

                match increment {
                    Expr::Empty => {}

                    _ => statements.push(Statement::mk_expression(increment)),
                }

                loop_block.push(Statement::mk_while(condition, statements));

                stmt = Statement::mk_block(loop_block);
            }

            TknK::Function => {
                self.consume(&TknK::Function);

                let id;
                let params;

                match self.expression(env)? {
                    Expr::Call { caller, args } => {
                        id = self.to_identifier(*caller)?;
                        params = self.to_identifiers(args)?;
                    }

                    _ => {
                        return Err(self.stumble(StumbleKind::Unexpected {
                            found: self.token().unwrap().kind.to_owned(),
                        }));
                    }
                };

                env.borrow_mut().insert(id.name(), ExprB::Nil);

                let lambda_env = Env::narrow(env.clone());
                {
                    let mut e = lambda_env.borrow_mut();
                    for p in &params {
                        e.insert(p.name(), ExprB::Nil);
                    }
                }

                let body = match self.statement(&lambda_env)? {
                    Statement::Block { statements } => statements,

                    _ => panic!("! Block expected"),
                };

                stmt = Statement::mk_function(id, params, body);
            }

            TknK::Semicolon => stmt = Statement::Empty,

            TknK::Break => {
                stmt = {
                    self.consume(&TknK::Break);
                    self.close_statement()?;
                    Statement::Break
                }
            }

            TknK::Return => {
                self.consume(&TknK::Return);
                let rexpr = self.expression_delimited(env, &TknK::Semicolon)?;
                self.consume(&TknK::Semicolon);
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
        delimiter: &TknK,
    ) -> Result<Expr, Stumble> {
        match self.expression(env) {
            Ok(e) => Ok(e),

            Err(e) => {
                if let StumbleKind::Unexpected { found } = e.kind() {
                    if found == delimiter {
                        Ok(Expr::Empty)
                    } else {
                        Err(e)
                    }
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn expression(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        self.assignment(env)
    }

    fn assignment(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        if let Some(TknK::Identifier { id }) = self.token_kind() {
            if let Some(TknK::Equal) = self.token_kind_ahead(1) {
                let offset = match env.borrow().offset(id) {
                    Some(d) => d,

                    None => {
                        panic!("! No offset found, hek");
                    }
                };

                let id = Expr::mk_identifier(id.to_owned(), Some(offset));

                unsafe { self.consume_unchecked() };
                self.consume(&TknK::Equal);
                let assignment = self.assignment(env)?;
                let expr = Expr::mk_assignment(id, assignment);

                return Ok(expr);
            }
        }

        self.logic_or(env)
    }

    fn logic_or(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        let mut expr = self.logic_and(env)?;

        while let Some(TknK::Or) = self.token_kind() {
            self.consume(&TknK::Or);

            let right = self.logic_and(env)?;
            expr = Expr::mk_or(expr, right);
        }

        Ok(expr)
    }

    fn logic_and(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        let mut expr = self.equality(env)?;

        while let Some(TknK::And) = self.token_kind() {
            self.consume(&TknK::And);
            let right = self.equality(env)?;
            expr = Expr::mk_and(expr, right);
        }

        Ok(expr)
    }

    fn equality(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        let mut expr = self.comparison(env)?;

        while let Some(token) = self.token() {
            match &token.kind {
                TknK::EqualEqual => {
                    self.consume(&TknK::EqualEqual);
                    let right = self.comparison(env)?;
                    expr = Expr::mk_binary(OpTwo::Eq, expr, right)
                }

                TknK::BangEqual => {
                    self.consume(&TknK::BangEqual);
                    let right = self.comparison(env)?;
                    expr = Expr::mk_binary(OpTwo::Neq, expr, right);
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        let mut expr = self.term(env)?;

        'comparison_match: while let Some(token) = self.token() {
            match &token.kind {
                TknK::Greater => {
                    self.consume(&TknK::Greater);
                    expr = Expr::mk_binary(OpTwo::Gt, expr, self.comparison(env)?)
                }

                TknK::GreaterEqual => {
                    self.consume(&TknK::GreaterEqual);
                    expr = Expr::mk_binary(OpTwo::Geq, expr, self.comparison(env)?)
                }

                TknK::Less => {
                    self.consume(&TknK::Less);
                    expr = Expr::mk_binary(OpTwo::Lt, expr, self.comparison(env)?)
                }

                TknK::LessEqual => {
                    self.consume(&TknK::LessEqual);
                    expr = Expr::mk_binary(OpTwo::Leq, expr, self.comparison(env)?)
                }

                _ => break 'comparison_match,
            }
        }

        Ok(expr)
    }

    fn term(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        let mut expr = self.factor(env)?;

        while let Some(token) = self.token() {
            match &token.kind {
                TknK::Minus => {
                    self.consume(&TknK::Minus);
                    expr = Expr::mk_binary(OpTwo::Minus, expr, self.term(env)?)
                }

                TknK::Plus => {
                    self.consume(&TknK::Plus);
                    expr = Expr::mk_binary(OpTwo::Plus, expr, self.term(env)?)
                }

                _ => break,
            }
        }
        Ok(expr)
    }

    fn factor(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        let mut expr = self.unary(env)?;

        while let Some(token) = self.token() {
            match &token.kind {
                TknK::Slash => {
                    self.consume(&TknK::Slash);
                    expr = Expr::mk_binary(OpTwo::Slash, expr, self.factor(env)?)
                }

                TknK::Star => {
                    self.consume(&TknK::Star);
                    expr = Expr::mk_binary(OpTwo::Star, expr, self.factor(env)?)
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn unary(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        match self.token() {
            None => Err(self.stumble(StumbleKind::MissingToken)),

            Some(token) => {
                let expr = match &token.kind {
                    TknK::Bang => {
                        self.consume(&TknK::Bang);
                        Expr::mk_unary(OpOne::Bang, self.unary(env)?)
                    }
                    TknK::Minus => {
                        self.consume(&TknK::Minus);
                        Expr::mk_unary(OpOne::Minus, self.unary(env)?)
                    }

                    _ => self.call(env)?,
                };

                Ok(expr)
            }
        }
    }

    #[allow(clippy::while_let_loop)]
    fn call(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        let mut expr = self.primary(env)?;

        loop {
            match self.token_kind() {
                Some(TknK::ParenL) => {
                    self.consume(&TknK::ParenL);
                    let mut args = Vec::default();
                    while self.token_kind().is_some_and(|kind| *kind != TknK::ParenR) {
                        args.push(self.expression(env)?);
                        if 255 <= args.len() {
                            return Err(self.stumble(StumbleKind::ArgLimit));
                        }

                        if let Some(TknK::Comma) = self.token_kind() {
                            self.consume(&TknK::Comma);
                        }
                    }

                    self.consume(&TknK::ParenR);

                    expr = Expr::mk_call(expr, args);
                }

                _ => break,
            }
        }

        Ok(expr)
    }

    fn primary(&mut self, env: &EnvHandle) -> Result<Expr, Stumble> {
        match self.token() {
            None => Err(self.stumble(StumbleKind::MissingToken)),

            Some(token) => {
                let expr = match &token.kind {
                    TknK::Number { literal } => Expr::mk_numeric(*literal),

                    TknK::String { literal } => Expr::mk_string(literal.to_owned()),

                    TknK::True => Expr::mk_true(),

                    TknK::False => Expr::mk_false(),

                    TknK::Nil => Expr::mk_nil(),

                    TknK::Identifier { id } => {
                        Expr::mk_identifier(id.to_owned(), env.borrow().offset(id))
                    }

                    TknK::ParenL => {
                        self.consume(&TknK::ParenL);
                        let expr = self.expression(env)?;
                        self.check_token(&TknK::ParenR);

                        expr
                    }

                    _ => {
                        return Err(self.stumble(StumbleKind::Unexpected {
                            found: token.kind.to_owned(),
                        }));
                    }
                };

                unsafe { self.consume_unchecked() };
                Ok(expr)
            }
        }
    }
}
