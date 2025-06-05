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

#[derive(Debug)]
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
        // # Safety: A global assignemt is always on the stack
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

    pub fn insert(&mut self, id: String, v: Value) -> Option<Value> {
        self.current_mut().insert(id, v)
    }

    pub fn assign(&mut self, id: &String, mut v: Value) -> Option<Value> {
        for assignment in self.stack.iter_mut().rev() {
            match assignment.get_mut(id) {
                Some(expr) => {
                    swap(expr, &mut v);
                    return Some(v);
                }

                None => continue,
            }
        }

        None
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
