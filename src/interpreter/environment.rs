use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::{collections::HashMap, mem::swap};

use super::ast::expression::ExprB;
use super::ast::{expression::Expr, identifier::Identifier, statement::Statements};

pub type Assignments = HashMap<Identifier, ExprB>;

pub type EnvHandle = Rc<RefCell<Env>>;

#[derive(Debug)]
pub struct Env {
    assignments: Assignments,
    depth: usize,
    enclosing: Option<EnvHandle>,
}

#[derive(Debug, PartialEq)]
pub enum EnvErr {
    MissingAsignee,
}

impl Default for Env {
    fn default() -> Self {
        Env {
            assignments: Assignments::default(),
            depth: 0,
            enclosing: None,
        }
    }
}

impl Env {
    pub fn fresh_global_handle() -> EnvHandle {
        Rc::new(RefCell::new(Env::default()))
    }

    pub fn fresh_std_env() -> EnvHandle {
        let global = Rc::new(RefCell::new(Env::default()));
        Env::narrow(global)
    }

    pub fn narrow(handle: EnvHandle) -> EnvHandle {
        let depth = handle.borrow().depth + 1;

        let narrow_env = Env {
            assignments: Assignments::default(),
            depth,
            enclosing: Some(handle),
        };

        Rc::new(RefCell::new(narrow_env))
    }

    pub fn enclosing(&self) -> Option<EnvHandle> {
        self.enclosing.clone()
    }
}

impl Env {
    pub fn insert(&mut self, id: Identifier, v: ExprB) -> Option<ExprB> {
        self.assignments.insert(id, v)
    }

    pub fn assign(&mut self, id: &Identifier, mut v: ExprB) -> Result<ExprB, EnvErr> {
        match self.assignments.get_mut(id) {
            Some(expr) => {
                swap(expr, &mut v);
                Ok(v)
            }

            None => match &self.enclosing {
                Some(e) => e.borrow_mut().assign(id, v),

                None => Err(EnvErr::MissingAsignee),
            },
        }
    }

    pub fn get(&self, id: &Identifier) -> Option<ExprB> {
        match self.assignments.get(id) {
            Some(v) => Some(v.clone()),

            None => match &self.enclosing {
                Some(e) => e.borrow().get(id),

                None => None,
            },
        }
    }
}
