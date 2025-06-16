use crate::interpreter::{
    Base, TreeWalker,
    ast::{
        expression::{Expr, ExprB, OpOne, OpTwo},
        identifier::Identifier,
        statement::Statement,
    },
    environment::{Env, EnvHandle},
    err::{Stumble, StumbleKind},
};

impl TreeWalker {
    pub fn eval_boolean(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        base: &mut Base,
    ) -> Result<bool, Stumble> {
        match self.eval(expr, env, base)? {
            ExprB::Boolean { b } => Ok(b),

            _ => Err(self.stumble_token(StumbleKind::ConflictingSubexpression)),
        }
    }

    pub fn eval_numeric(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        base: &mut Base,
    ) -> Result<f64, Stumble> {
        match self.eval(expr, env, base)? {
            ExprB::Numeric { n } => Ok(n),

            ExprB::String { s } => {
                if let Ok(result) = s.parse::<f64>() {
                    Ok(result)
                } else {
                    panic!("Failed to convert string to numeric")
                }
            }

            e => panic!("{e:?}"),
        }
    }

    pub fn eval_string(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        base: &mut Base,
    ) -> Result<String, Stumble> {
        match self.eval(expr, env, base)? {
            ExprB::String { s } => Ok(s.to_owned()),

            _ => Err(self.stumble_token(StumbleKind::ConflictingSubexpression)),
        }
    }

    pub fn get_identifier(&self, expr: Expr) -> Result<Identifier, Stumble> {
        match expr {
            Expr::Identifier { id: i } => Ok(i),

            _ => Err(self.stumble_token(StumbleKind::InvalidAssignTo)),
        }
    }

    pub fn eval(&self, expr: &Expr, env: &EnvHandle, base: &mut Base) -> Result<ExprB, Stumble> {
        let value = match expr {
            Expr::Empty => ExprB::Nil,

            Expr::Basic(b_expr) => b_expr.clone(),

            Expr::Identifier { id } => match env.borrow().get(id) {
                None => {
                    return Err(self.stumble_token(StumbleKind::InvalidIdentifier {
                        id: id.name.clone(),
                    }));
                }

                Some(e) => return Ok(e.to_owned()),
            },

            Expr::Assignment {
                id: name,
                e: assignment,
            } => {
                let assignment = self.eval(assignment, env, base)?;

                let id = self.get_identifier(*name.clone())?;

                match env.borrow_mut().assign(id.name(), assignment.clone()) {
                    Ok(_) => {}

                    Err(e) => return Err(self.stumble_token(e)),
                };

                assignment
            }

            Expr::Grouping { e } => self.eval(e, env, base)?,

            Expr::Unary { op, e } => {
                use OpOne::*;
                match op {
                    Minus => ExprB::mk_numeric(-self.eval_numeric(e, env, base)?),

                    Bang => ExprB::mk_bool(!(self.eval_boolean(e, env, base)?)),
                }
            }

            Expr::Binary { op, a: l, b: r } => {
                use OpTwo::*;
                match op {
                    Minus => ExprB::mk_numeric(
                        self.eval_numeric(l, env, base)? - self.eval_numeric(r, env, base)?,
                    ),

                    Slash => ExprB::mk_numeric(
                        self.eval_numeric(l, env, base)? / self.eval_numeric(r, env, base)?,
                    ),

                    Star => ExprB::mk_numeric(
                        self.eval_numeric(l, env, base)? * self.eval_numeric(r, env, base)?,
                    ),

                    Plus => match (self.eval(l, env, base)?, self.eval(r, env, base)?) {
                        (ExprB::Numeric { n: l }, ExprB::Numeric { n: r }) => {
                            ExprB::mk_numeric(l + r)
                        }

                        (ExprB::String { s: mut l }, ExprB::String { s: r }) => {
                            l.push_str(r.as_str());
                            ExprB::mk_string(l)
                        }

                        _ => {
                            return Err(self.stumble_token(StumbleKind::ConflictingSubexpression));
                        }
                    },

                    Gt => ExprB::mk_bool(
                        self.eval_numeric(l, env, base)? > self.eval_numeric(r, env, base)?,
                    ),

                    Geq => ExprB::mk_bool(
                        self.eval_numeric(l, env, base)? >= self.eval_numeric(r, env, base)?,
                    ),

                    Lt => ExprB::mk_bool(
                        self.eval_numeric(l, env, base)? < self.eval_numeric(r, env, base)?,
                    ),

                    Leq => ExprB::mk_bool(
                        self.eval_numeric(l, env, base)? <= self.eval_numeric(r, env, base)?,
                    ),

                    Eq => ExprB::mk_bool(self.eval(l, env, base)? == self.eval(r, env, base)?),

                    Neq => ExprB::mk_bool(self.eval(l, env, base)? != self.eval(r, env, base)?),
                }
            }

            Expr::Or { a, b } => {
                let a_value = self.eval(a, env, base)?;

                if a_value.is_truthy() {
                    a_value
                } else {
                    self.eval(b, env, base)?
                }
            }

            Expr::And { a, b } => {
                let a_value = self.eval(a, env, base)?;

                if a_value.is_falsey() {
                    a_value
                } else {
                    self.eval(b, env, base)?
                }
            }

            Expr::Call { caller, args } => {
                match self.eval(caller, env, base)? {
                    ExprB::Lambda {
                        env: lenv,
                        params,
                        body,
                    } => {
                        // TODO: Write the args to the same env as the body?

                        let args_env = Env::narrow(lenv);
                        for (id, v) in params.iter().zip(args.iter()) {
                            let bv = self.eval(v, env, base)?;
                            args_env.borrow_mut().insert(id.name(), bv);
                        }

                        let body_env = Env::narrow(args_env);
                        for statement in &body {
                            let expr = self.interpret(statement, &body_env, base)?;
                            if let Statement::Return { .. } = statement {
                                return Ok(expr.1);
                            }
                        }
                    }

                    _ => return Err(self.stumble_token(StumbleKind::ExpectedLambda)),
                }

                ExprB::Nil
            }
        };

        Ok(value)
    }
}
