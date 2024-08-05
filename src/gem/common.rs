use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    LitNum(f64),
    LitStr(String),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    Colon,
    Comma,
    Dot,
    Semicolon,
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    Func,
    Var,
    Eof,
}

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Expr {
    LitNum(f64),
    LitStr(String),
    Variable(String),
    OpBinary(Operation, Box<Expr>, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
}

pub enum Stmt {
    FunctionDecl(String, Vec<String>, Expr),
    VariableDecl(String, Expr),
    Program(Vec<Stmt>),
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            Operation::Add => "+",
            Operation::Sub => "-",
            Operation::Mul => "*",
            Operation::Div => "/",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::LitNum(value) => write!(f, "{}", value),
            Expr::LitStr(value) => write!(f, "str'{}'", value),
            Expr::Variable(name) => write!(f, "{}", name),
            Expr::OpBinary(op, left, right) => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expr::FunctionCall(name, args) => {
                let args_str: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
                write!(f, "{}({})", name, args_str.join(", "))
            }
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::FunctionDecl(name, params, body) => {
                let params_str = params.join(", ");
                write!(f, "function {}({}) {{\n    {}\n}}", name, params_str, body)
            }
            Stmt::VariableDecl(name, value) => {
                write!(f, "variable {}({}) ", name, value)
            }
            Stmt::Program(stmts) => {
                for stmt in stmts {
                    writeln!(f, "{}\n", stmt)?;
                }
                Ok(())
            }
        }
    }
}