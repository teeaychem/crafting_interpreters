use std::io::Write;

use crate::interpreter::{
    Base, Interpreter,
    ast::{
        expression::{Expr, ExprB, OpOne, OpTwo},
        identifier::Identifier,
        statement::Statement,
    },
    environment::{Env, EnvHandle},
    evaluation::value::EvalErr,
};

impl Interpreter {
    pub fn eval_boolean(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        base: &mut Base,
    ) -> Result<bool, EvalErr> {
        match self.eval(expr, env, base)? {
            ExprB::Boolean { b } => Ok(b),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn eval_numeric(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        base: &mut Base,
    ) -> Result<f64, EvalErr> {
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
    ) -> Result<String, EvalErr> {
        match self.eval(expr, env, base)? {
            ExprB::String { s } => Ok(s.to_owned()),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn get_identifier(&self, expr: &Expr) -> Result<Identifier, EvalErr> {
        match expr {
            Expr::Identifier { id: i } => Ok(i.to_owned()),

            _ => Err(EvalErr::InvalidAsignTo),
        }
    }

    pub fn eval(&self, expr: &Expr, env: &EnvHandle, base: &mut Base) -> Result<ExprB, EvalErr> {
        let value = match expr {
            Expr::Empty => ExprB::Nil,

            Expr::Basic(b_expr) => b_expr.clone(),

            Expr::Identifier { id } => match env.borrow().get(id) {
                None => {
                    println!("Id `{id}` not found in the following env:");
                    println!("{:?}", env);
                    return Err(EvalErr::InvalidIdentifier { id: id.to_owned() });
                }

                Some(e) => return Ok(e.to_owned()),
            },

            Expr::Assignment {
                id: name,
                e: assignment,
            } => {
                let assignment = self.eval(assignment, env, base)?;

                let name = self.get_identifier(name)?;

                match env.borrow_mut().assign(&name, assignment.clone()) {
                    Ok(_) => {}

                    Err(e) => return Err(EvalErr::EnvErr { err: e }),
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

                        _ => return Err(EvalErr::ConflictingSubexpression),
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
                        let mut return_expr = None;

                        let eenv = Env::narrow(lenv);
                        for (id, v) in params.iter().zip(args.iter()) {
                            let bv = self.eval(v, env, base)?;
                            eenv.borrow_mut().insert(id.to_owned(), bv);
                        }

                        for statement in &body {
                            return_expr = self.interpret(statement, &eenv, base)?;
                            if let Statement::Return { .. } = statement {
                                if let Some(v) = return_expr {
                                    return Ok(v);
                                }
                                break;
                            }
                        }
                    }

                    _ => panic!("! Expected lambda"),
                }

                ExprB::Nil
            }
        };

        Ok(value)
    }
}
