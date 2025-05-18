use crate::ast::statement::Statements;

use super::value::ValueError;

pub trait Interpret {
    fn interpret(&self) -> Result<(), ValueError>;
}

pub fn interpret(statements: &Statements) {
    for statement in statements {
        statement.interpret();
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner::Scanner};

    use super::*;

    #[test]
    fn print() {
        let mut scannar = Scanner::default();
        let input = "print 5 + 5";

        scannar.scan(input);

        let mut parser = Parser::from(scannar);

        parser.parse();

        interpret(&parser.statements);
    }
}
