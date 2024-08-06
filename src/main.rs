use gem::lexer::*;
use gem::parser::*;

fn main() {
    const SOURCE: &str = r#"
        var x = "Hello, From x!";
        var y = 3.14;

        func add(x, y) {
            x + y;
        }

        func foo(z, x) {
            var w = 12;

            z * add(x, z);
        }

        func main() {
            foo(bar, 12);
        }
    "#;

    let lexer = Lexer::new(SOURCE);
    let mut parser = Parser::new(lexer);

    let parsed = parser.parse();

    println!("{}", parsed);
}