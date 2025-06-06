use crate::interpreter::ast::expression::ExprB;
use crate::interpreter::ast::identifier::Identifier;

use crate::{
    Interpreter,
    interpreter::{
        ast::expression::{Expr, OpOne, OpTwo},
        environment::EnvHandle,
    },
};

use crate::interpreter::evaluation::value::EvalErr;

impl Interpreter {
    pub fn eval_boolean(&self, expr: &Expr, env: &EnvHandle) -> Result<bool, EvalErr> {
        match self.eval(expr, env)? {
            ExprB::Boolean { b } => Ok(b),

            _ => Err(EvalErr::ConflictingSubexpression),
        }
    }

    pub fn eval_numeric(&self, expr: &Expr, env: &EnvHandle) -> Result<f64, EvalErr> {
        match self.eval(expr, env)? {
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

    pub fn eval_string(&self, expr: &Expr, env: &EnvHandle) -> Result<String, EvalErr> {
        match self.eval(expr, env)? {
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

    pub fn eval(&self, expr: &Expr, env: &EnvHandle) -> Result<ExprB, EvalErr> {
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
                let assignment = self.eval(assignment, env)?;

                let name = self.get_identifier(name)?;

                match env.borrow_mut().assign(&name, assignment.clone()) {
                    Ok(_) => {}

                    Err(e) => return Err(EvalErr::EnvErr { err: e }),
                };

                assignment
            }

            Expr::Grouping { e } => self.eval(e, env)?,

            Expr::Unary { op, e } => {
                use OpOne::*;
                match op {
                    Minus => ExprB::mk_numeric(-self.eval_numeric(e, env)?),

                    Bang => ExprB::mk_bool(!(self.eval_boolean(e, env)?)),
                }
            }

            Expr::Binary { op, a: l, b: r } => {
                use OpTwo::*;
                match op {
                    Minus => {
                        ExprB::mk_numeric(self.eval_numeric(l, env)? - self.eval_numeric(r, env)?)
                    }

                    Slash => {
                        ExprB::mk_numeric(self.eval_numeric(l, env)? / self.eval_numeric(r, env)?)
                    }

                    Star => {
                        ExprB::mk_numeric(self.eval_numeric(l, env)? * self.eval_numeric(r, env)?)
                    }

                    Plus => match (self.eval(l, env)?, self.eval(r, env)?) {
                        (ExprB::Numeric { n: l }, ExprB::Numeric { n: r }) => {
                            ExprB::mk_numeric(l + r)
                        }

                        (ExprB::String { s: mut l }, ExprB::String { s: r }) => {
                            l.push_str(r.as_str());
                            ExprB::mk_string(l)
                        }

                        _ => return Err(EvalErr::ConflictingSubexpression),
                    },

                    Gt => ExprB::mk_bool(self.eval_numeric(l, env)? > self.eval_numeric(r, env)?),

                    Geq => ExprB::mk_bool(self.eval_numeric(l, env)? >= self.eval_numeric(r, env)?),

                    Lt => ExprB::mk_bool(self.eval_numeric(l, env)? < self.eval_numeric(r, env)?),

                    Leq => ExprB::mk_bool(self.eval_numeric(l, env)? <= self.eval_numeric(r, env)?),

                    Eq => ExprB::mk_bool(self.eval(l, env)? == self.eval(r, env)?),

                    Neq => ExprB::mk_bool(self.eval(l, env)? != self.eval(r, env)?),
                }
            }

            Expr::Or { a, b } => {
                let a_value = self.eval(a, env)?;

                if a_value.is_truthy() {
                    a_value
                } else {
                    self.eval(b, env)?
                }
            }

            Expr::And { a, b } => {
                let a_value = self.eval(a, env)?;

                if a_value.is_falsey() {
                    a_value
                } else {
                    self.eval(b, env)?
                }
            }

            Expr::Call { callee, args } => {
                todo!("{expr}")
            }
        };

        Ok(value)
    }
}
