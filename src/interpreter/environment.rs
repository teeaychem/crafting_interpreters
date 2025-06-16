use std::{cell::RefCell, collections::HashMap, mem::swap, rc::Rc};

use super::ast::{
    expression::ExprB,
    identifier::{Id, Identifier},
};

pub type Assignments = HashMap<Id, ExprB>;

pub type EnvHandle = Rc<RefCell<Env>>;

#[derive(Clone, Debug)]
pub struct Env {
    assignments: Assignments,
    enclosing: Option<EnvHandle>,

    depth: usize,
}

#[derive(Debug, PartialEq)]
pub enum EnvErr {
    MissingAsignee,
}

impl Default for Env {
    fn default() -> Self {
        Env {
            assignments: Assignments::default(),
            enclosing: None,

            depth: 0,
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
        let enclosing = Some(handle.clone());

        let narrow_env = Env {
            assignments: Assignments::default(),
            enclosing,
            depth,
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
            None => None,

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
                let mut enclosing_env = self.enclosing();

                loop {
                    match &enclosing_env {
                        Some(ex) => {
                            if ex.borrow().assignments.contains_key(id) {
                                return Some(offset);
                            }
                        }

                        None => return None,
                    }

                    offset += 1;
                    enclosing_env = enclosing_env.unwrap().borrow().enclosing();
                }
            }
        }
    }
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Env @ {}", self.depth);
        for key in self.assignments.keys() {
            writeln!(f, "\t{key}");
        }

        Ok(())
    }
}
