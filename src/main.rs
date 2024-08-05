use gem::lexer::*;
use gem::parser::*;

fn main() {
    let source = r#"
        var x = "Hello, From x!";
        var y = 3.14;
        func greet() {
            "Hello, world!"
        }
    "#;

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let parsed = parser.parse();

    println!("{}", parsed);
}
