use std::os::macos::raw::stat;

use super::{Statement, Statements};

pub fn interpret(statements: &Statements) {
    for statement in statements {
        statement.interpret();
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, scanner::Scanner, statement::interpret};

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
