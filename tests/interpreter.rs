#[cfg(test)]
mod interpreter {
    use std::io::BufWriter;

    use loxy_lib::{Interpreter, Parser, Scanner, interpreter::environment::Env};

    fn test_io(input: &str, output: &str) {
        let mut scanner = Scanner::default();
        let mut parser = Parser::default();

        scanner.scan(input);
        parser.consume_scanner(scanner);
        match parser.parse() {
            Ok(_) => {}

            Err(e) => {
                println!("Parser failure: {e:?}");
                dbg!(&parser);
                panic!();
            }
        };

        let mut buffer = Vec::with_capacity(output.len());
        let mut stream = BufWriter::new(&mut buffer);

        {
            let interpreter = Interpreter::default();
            let mut env = Env::default();

            // interpreter.set_destination();

            match interpreter.interpret_all(parser.statements(), &mut env, &mut stream) {
                Ok(_) => {}

                Err(e) => panic!("Interpretation error: {e:?}"),
            };
        }

        let buffer_string = std::str::from_utf8(&stream.buffer());

        assert_eq!(buffer_string.expect("Failed to interpret").trim(), output);
    }

    #[allow(dead_code)]
    fn interpret_stdout(input: &str) {
        let mut scanner = Scanner::default();
        let mut parser = Parser::default();

        scanner.scan(input);
        parser.consume_scanner(scanner);
        match parser.parse() {
            Ok(_) => {}

            Err(e) => {
                println!("Parser failure: {e:?}");
                dbg!(&parser);
                panic!();
            }
        };

        let interpreter = Interpreter::default();
        let mut env = Env::default();

        match interpreter.interpret_all(parser.statements(), &mut env, &mut std::io::stdout()) {
            Ok(_) => {}

            Err(e) => panic!("Interpretation error: {e:?}"),
        };
    }

    #[test]
    fn print() {
        test_io("print 5 + 5; print 5 - 5; ", "10\n0");

        test_io("print true;", "true");

        test_io("print !true;", "false");
    }

    #[test]
    fn print_string() {
        let input = r#"print "print";"#;

        test_io(input, "print");
    }

    #[test]
    fn declaration() {
        let input = r#"
var test = "testing";
test = "testing again";
print test;"#;

        test_io(input, "testing again");
    }

    #[test]
    fn nested() {
        let input = r#"
var a = "a";
var b = "b";
a = b = "c";
print a;
"#;

        test_io(input, "c");
    }

    #[test]
    fn print_addition() {
        let input = "
var a = 3;
var b = 3;
print (a * b) / (a + b);
";
        test_io(input, "1.5");
    }

    #[test]
    fn var_nil() {
        let input = "
var a;
print a;
";
        test_io(input, "nil");
    }

    #[test]
    fn block_basic() {
        let input = "
var a;
print a;
{
  a = 1;
  print a;
}
print a;
";
        test_io(input, "nil\n1\n1");
    }

    #[test]
    fn block_nested() {
        let input = r#"
var a = "global a";
var b = "global b";
var c = "global c";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;
"#;
        test_io(
            input,
            "inner a
outer b
global c
outer a
outer b
global c
global a
global b
global c",
        );
    }

    #[test]
    fn challenge() {
        let input = r#"
var a = 1;
{
   var a = a + 2;
   print a;
}
"#;

        test_io(input, "3");
    }

    #[test]
    fn conditional() {
        let input = r#"
if (false != true)
  print "ok";
"#;
        test_io(input, "ok");

        let input = r#"
if (false == true)
  print "nok";
"#;
        test_io(input, "");

        let input = r#"
if (false == true)
  print "nok";
else
  print "ok";
"#;
        test_io(input, "ok");
    }

    #[test]
    fn logic_or() {
        let input = r#"
if (true or true)
  print "ok";
"#;
        test_io(input, "ok");

        let input = r#"
if (true or false)
  print "ok";
"#;
        test_io(input, "ok");

        let input = r#"
if (false or true)
  print "ok";
"#;
        test_io(input, "ok");

        let input = r#"
if (false or false)
  print "nok";
else
  print "ok";
"#;
        test_io(input, "ok");
    }

    #[test]
    fn logic_and() {
        let input = r#"
if (true and true)
  print "ok";
"#;
        test_io(input, "ok");

        let input = r#"
if (true and false)
  print "nok";
else
  print "ok";
"#;
        test_io(input, "ok");

        let input = r#"
if (false and true)
  print "nok";
else
  print "ok";
"#;
        test_io(input, "ok");

        let input = r#"
if (false and false)
  print "nok";
else
  print "ok";
"#;
        test_io(input, "ok");
    }

    #[test]
    fn logic_mix() {
        let input = r#"
var a = 2 * 2;
if ((a or false) and 2 / 2 == 1)
  print a;
"#;
        test_io(input, "4");
    }

    #[test]
    fn while_simple() {
        let input = r#"
var a = 1;
while (a < 4) {
  print a;
  a = a + 1;
}
print a;
"#;
        test_io(input, "1\n2\n3\n4");
    }

    #[test]
    fn for_simple() {
        let input = r#"
for (var a = 1; a < 5; a = a + 1) {
  print a;
}
"#;
        test_io(input, "1\n2\n3\n4");
    }

    #[test]
    fn for_fibonacci() {
        let input = r#"
var a = 0;
var temp;
for (var b = 1; a < 150; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
"#;
        test_io(input, "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n144");
    }

    #[test]
    fn basic_call() {
        let input = r#"
add(1, 2);
"#;
        test_io(input, "");
    }

    #[test]
    fn basic_call_call() {
        let input = r#"
add(1, 2)(3);
"#;
        test_io(input, "");
    }

    #[test]
    fn empty_call() {
        let input = r#"
empty_call();
"#;
        test_io(input, "");
    }
}
