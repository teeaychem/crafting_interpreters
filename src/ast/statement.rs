use crate::{
    ast::expression::Expression,
    parser::{evaluate::Evaluate, interpret::Interpret, value::ValueError},
};

pub type Statements = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Expression { e: Expression },
    Print { e: Expression },
}

impl Interpret for Statement {
    fn interpret(&self) -> Result<(), ValueError> {
        match self {
            Statement::Print { e } => println!("{}", e.evaluate()?),

            _ => todo!(),
        }

        Ok(())
    }
}
