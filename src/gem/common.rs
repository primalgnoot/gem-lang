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
    If,
    Else,
    While,
    For,
    Return,
    Eof,
}

#[derive(Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone)]
pub enum Expr {
    Null,
    LitNum(f64),
    LitStr(String),
    Variable(String),
    OpBinary(Operation, Box<Expr>, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
}

#[derive(Clone)]
pub enum Stmt {
    FunctionDecl(Expr, String, Vec<String>, Box<Stmt>),
    VariableDecl(String, Expr),
    Program(Vec<Stmt>),
    Expr(Expr),
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
            Expr::Null => write!(f, "null"),
            Expr::LitNum(value) => write!(f, "{}", value),
            Expr::LitStr(value) => write!(f, "'{}'", value),
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
            Stmt::FunctionDecl(returns, name, params, body) => {
                let params_str = params.join(", ");
                writeln!(f, "function {}({}) -> {} {{", name, params_str, returns)?;
                write_indented_stmt(f, body, 1)?;
                write!(f, "}}")
            }
            Stmt::VariableDecl(name, value) => {
                write!(f, "variable {}({})", name, value)
            }
            Stmt::Expr(expr) => {
                write!(f, "expr({})", expr)
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

fn write_indented_stmt(f: &mut fmt::Formatter<'_>, stmt: &Stmt, level: usize) -> fmt::Result {
    let indent = "    ".repeat(level);
    match stmt {
        Stmt::FunctionDecl(_, _, _, body) => {
            if let Stmt::Program(stmts) = &**body {
                for stmt in stmts {
                    writeln!(f, "{}{}", indent, stmt)?;
                }
            }
        }
        Stmt::VariableDecl(name, value) => {
            writeln!(f, "{}variable {}({})", indent, name, value)?
        }
        Stmt::Expr(expr) => {
            writeln!(f, "{}expr({})", indent, expr)?
        }
        Stmt::Program(stmts) => {
            for stmt in stmts {
                write_indented_stmt(f, stmt, level)?
            }
        }
    }
    Ok(())
}