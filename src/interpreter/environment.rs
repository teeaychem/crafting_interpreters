use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::{collections::HashMap, mem::swap};

use crate::interpreter::parser::value::Value;

use super::ast::{
    expression::Expression, identifier::Identifier, literal::Literal, statement::Statements,
};

pub type Assignments = HashMap<Identifier, Value>;

pub struct Function {
    name: String,
    arity: usize,
    args: Vec<Identifier>,
    task: Statements,
}

pub type EnvHandle = Rc<RefCell<Env>>;

#[derive(Debug)]
pub struct Env {
    assignments: Assignments,
    depth: usize,
    enclosing: Option<EnvHandle>,
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

    pub fn narrow(handle: EnvHandle) -> EnvHandle {
        let depth = handle.borrow().depth + 1;

        let narrow_env = Env {
            assignments: Assignments::default(),
            depth,
            enclosing: Some(handle),
        };

        Rc::new(RefCell::new(narrow_env))
    }
}

impl Env {
    pub fn current_mut(&mut self) -> &mut Assignments {
        &mut self.assignments
    }

    pub fn insert(&mut self, id: String, v: Value) -> Option<Value> {
        self.current_mut().insert(id, v)
    }

    pub fn assign(&mut self, id: &String, mut v: Value) -> Option<Value> {
        match self.assignments.get_mut(id) {
            Some(expr) => {
                swap(expr, &mut v);
                Some(v)
            }

            None => match &self.enclosing {
                Some(e) => e.borrow_mut().assign(id, v),

                None => None,
            },
        }
    }

    pub fn get(&self, id: &String) -> Option<Value> {
        match self.assignments.get(id) {
            Some(v) => Some(v.clone()),

            None => match &self.enclosing {
                Some(e) => e.borrow().get(id),

                None => None,
            },
        }
    }
}
