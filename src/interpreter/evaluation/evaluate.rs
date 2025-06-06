use std::io::Write;

use crate::interpreter::ast::expression::ExprB;
use crate::interpreter::ast::identifier::Identifier;

use crate::interpreter::environment::Env;
use crate::{
    Interpreter,
    interpreter::{
        ast::expression::{Expr, OpOne, OpTwo},
        environment::EnvHandle,
    },
};

use crate::interpreter::evaluation::value::EvalErr;

impl Interpreter {
    pub fn eval_boolean<W: Write>(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        out: &mut W,
    ) -> Result<bool, EvalErr> {
        match self.eval(expr, env, out)? {
            ExprB::Boolean { b } => Ok(b),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn eval_numeric<W: Write>(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        out: &mut W,
    ) -> Result<f64, EvalErr> {
        match self.eval(expr, env, out)? {
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

    pub fn eval_string<W: Write>(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        out: &mut W,
    ) -> Result<String, EvalErr> {
        match self.eval(expr, env, out)? {
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

    pub fn eval<W: Write>(
        &self,
        expr: &Expr,
        env: &EnvHandle,
        out: &mut W,
    ) -> Result<ExprB, EvalErr> {
        let value = match expr {
            Expr::Empty => ExprB::Nil,

            Expr::Basic(bexpr) => {
                //
                bexpr.clone()
            }

            Expr::Identifier { id } => match env.borrow().get(id) {
                None => {
                    println!("{:?}", env);
                    return Err(EvalErr::InvalidIdentifier { id: id.to_owned() });
                }

                Some(e) => return Ok(e.to_owned()),
            },

            Expr::Assignment {
                id: name,
                e: assignment,
            } => {
                let assignment = self.eval(assignment, env, out)?;

                let name = self.get_identifier(name)?;

                match env.borrow_mut().assign(&name, assignment.clone()) {
                    Ok(_) => {}

                    Err(e) => return Err(EvalErr::EnvErr { err: e }),
                };

                assignment
            }

            Expr::Grouping { e } => self.eval(e, env, out)?,

            Expr::Unary { op, e } => {
                use OpOne::*;
                match op {
                    Minus => ExprB::mk_numeric(-self.eval_numeric(e, env, out)?),

                    Bang => ExprB::mk_bool(!(self.eval_boolean(e, env, out)?)),
                }
            }

            Expr::Binary { op, a: l, b: r } => {
                use OpTwo::*;
                match op {
                    Minus => ExprB::mk_numeric(
                        self.eval_numeric(l, env, out)? - self.eval_numeric(r, env, out)?,
                    ),

                    Slash => ExprB::mk_numeric(
                        self.eval_numeric(l, env, out)? / self.eval_numeric(r, env, out)?,
                    ),

                    Star => ExprB::mk_numeric(
                        self.eval_numeric(l, env, out)? * self.eval_numeric(r, env, out)?,
                    ),

                    Plus => match (self.eval(l, env, out)?, self.eval(r, env, out)?) {
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
                        self.eval_numeric(l, env, out)? > self.eval_numeric(r, env, out)?,
                    ),

                    Geq => ExprB::mk_bool(
                        self.eval_numeric(l, env, out)? >= self.eval_numeric(r, env, out)?,
                    ),

                    Lt => ExprB::mk_bool(
                        self.eval_numeric(l, env, out)? < self.eval_numeric(r, env, out)?,
                    ),

                    Leq => ExprB::mk_bool(
                        self.eval_numeric(l, env, out)? <= self.eval_numeric(r, env, out)?,
                    ),

                    Eq => ExprB::mk_bool(self.eval(l, env, out)? == self.eval(r, env, out)?),

                    Neq => ExprB::mk_bool(self.eval(l, env, out)? != self.eval(r, env, out)?),
                }
            }

            Expr::Or { a, b } => {
                let a_value = self.eval(a, env, out)?;

                if a_value.is_truthy() {
                    a_value
                } else {
                    self.eval(b, env, out)?
                }
            }

            Expr::And { a, b } => {
                let a_value = self.eval(a, env, out)?;

                if a_value.is_falsey() {
                    a_value
                } else {
                    self.eval(b, env, out)?
                }
            }

            Expr::Call { caller, args } => {
                match self.eval(caller, env, out)? {
                    ExprB::Lambda {
                        env: lenv,
                        params,
                        body,
                    } => {
                        let eenv = Env::narrow(lenv);
                        for (id, v) in params.iter().zip(args.iter()) {
                            let bv = self.eval(v, env, out)?;
                            eenv.borrow_mut().insert(id.to_owned(), bv);
                        }

                        for statement in &body {
                            self.interpret(statement, &eenv, out);
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
