use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    mem::swap,
    rc::Rc,
};

use super::ast::{
    expression::{Expr, ExprB},
    identifier::{Id, Identifier},
    statement::Statements,
};

pub type Assignments = HashMap<Id, ExprB>;

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

    pub fn depth(&self) -> usize {
        self.depth
    }
}

impl Env {
    pub fn insert(&mut self, id: impl std::borrow::Borrow<Id>, v: ExprB) -> Option<ExprB> {
        self.assignments.insert(id.borrow().clone(), v)
    }

    pub fn assign(&mut self, id: &Id, mut v: ExprB) -> Result<ExprB, EnvErr> {
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
        match id.offset() {
            None => panic!("! No offset"),

            Some(0) => self.assignments.get(id.name()).cloned(),

            Some(mut o) => {
                let mut ee = self.enclosing();
                while 1 < o {
                    ee = ee.expect("! Missing env").borrow_mut().enclosing();
                    o -= 1;
                }

                ee.expect("X_X")
                    .borrow_mut()
                    .assignments
                    .get(id.name())
                    .cloned()
            }
        }
    }

    pub fn offset(&self, id: &Id) -> Option<usize> {
        match self.assignments.get(id) {
            Some(_) => Some(0),

            None => {
                let mut offset = 1;
                let mut ee = self.enclosing();

                loop {
                    match &ee {
                        Some(ex) => {
                            if ex.borrow().assignments.contains_key(id) {
                                return Some(offset);
                            }
                        }

                        None => return None,
                    }

                    offset += 1;
                    ee = ee.unwrap().borrow().enclosing();
                }

                None
            }
        }
    }
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Env @ {}", self.depth);
        for (k, v) in &self.assignments {
            writeln!(f, "\t{k}");
        }

        Ok(())
    }
}
