use std::string::ParseError;

use crate::{
    expression::Expression,
    value::{Evaluable, ValueError},
};

mod interpret;

type Statements = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Expression { e: Expression },
    Print { e: Expression },
}

impl Statement {
    pub fn interpret(&self) -> Result<(), ValueError> {
        match self {
            Statement::Print { e } => println!("{}", e.evaluate()?),

            _ => todo!(),
        }

        Ok(())
    }
}
