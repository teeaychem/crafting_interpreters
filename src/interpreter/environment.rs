use std::collections::HashMap;

use crate::parser::value::Value;

pub type Assignments = HashMap<String, Value>;

pub struct Env {
    stack: Vec<Assignments>,
}

impl Default for Env {
    fn default() -> Self {
        Env {
            stack: vec![Assignments::default()],
        }
    }
}

impl Env {
    pub fn current_mut(&mut self) -> &mut Assignments {
        // # Safety: A global assignemt is on the stack
        unsafe { self.stack.last_mut().unwrap_unchecked() }
    }

    pub fn narrow(&mut self) {
        self.stack.push(Assignments::default());
    }

    pub fn relax(&mut self) {
        if 1 < self.stack.len() {
            self.stack.pop();
        } else {
            panic!("! Attempt to relax global assignments")
        }
    }

    pub fn insert(&mut self, id: String, v: &Value) -> Option<Value> {
        self.current_mut().insert(id, v.to_owned())
    }

    pub fn get(&self, id: &String) -> Option<&Value> {
        for assignment in self.stack.iter().rev() {
            match assignment.get(id) {
                Some(expr) => return Some(expr),

                None => continue,
            }
        }

        None
    }
}
