use gem::lexer::*;
use gem::parser::*;

fn main() {
    let source = "
        var gx = 32;
        var gz = 64;

        func add(x, y) {
            x + y
        }

        func foo(z, x) {
            z * add(x, z)
        }
    ";

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let evaluated = parser.parse();

    println!("{}", evaluated);
}