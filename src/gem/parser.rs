use std::fmt;
use crate::lexer::*;

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Stmt {
    FunctionDecl(String, Vec<String>, Expr),
    VariableDecl(String, Expr),
    Program(Vec<Stmt>),
}

pub enum Expr {
    Number(f64),
    Variable(String),
    OpBinary(Operation, Box<Expr>, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::Eof,
        };
        parser.next_token(); // Initialize the first token
        parser
    }

    #[inline(always)]
    fn expect_token(&mut self, token: Token) {
        if self.current_token != token {
            panic!("Expected token: {:?}, got: {:?}", token, self.current_token)
        }
    }

    #[inline(always)]
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    // Expects a token and consumes if valid
    #[inline(always)]
    fn consume(&mut self, token: Token) {
        self.expect_token(token);
        self.next_token();
    }

    fn parse_function_decl(&mut self) -> Stmt {
        self.consume(Token::Func);

        let name = if let Token::Identifier(ref id) = self.current_token {
            id.clone()
        } else {
            panic!("Expected function name");
        };
    
        self.next_token(); // Consume function name
    
        self.consume(Token::ParenL);
    
        let mut params = Vec::new();
        while self.current_token != Token::ParenR {
            if let Token::Identifier(ref id) = self.current_token {
                params.push(id.clone());
            } else {
                panic!("Expected parameter name");
            }
            self.next_token();
            if self.current_token == Token::Comma {
                self.next_token();
            }
        }
    
        self.consume(Token::ParenR);
        self.consume(Token::BraceL);
    
        let body = self.parse_expression(); // Parse the function body

        self.consume(Token::BraceR);

        Stmt::FunctionDecl(name, params, body)
    }

    fn parse_variable_decl(&mut self) -> Stmt {
        self.expect_token(Token::Var);
        self.next_token(); // Consume 'var' keyword
    
        let name = if let Token::Identifier(ref id) = self.current_token {
            id.clone()
        } else {
            panic!("Expected variable name");
        };
    
        self.next_token(); // Consume variable name
    
        let mut value = None;
    
        if self.current_token == Token::Eq {
            self.next_token(); // Consume '='
            value = Some(self.parse_expression());
        }
    
        self.expect_token(Token::Semicolon);
        self.next_token(); // Consume semicolon
    
        Stmt::VariableDecl(name, value.unwrap_or(Expr::Number(0.0)))
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_binop_expr(0)
    }

    fn parse_binop_expr(&mut self, precedence: u8) -> Expr {
        let mut expr = self.parse_primary_expr();

        while let Some(op) = self.current_op(precedence) {
            self.next_token(); // Consume the operator
            let mut rhs = self.parse_primary_expr();
            
            while let Some(next_op) = self.current_op(precedence) {
                if precedence >= self.precedence(next_op) {
                    rhs = self.parse_binop_expr(precedence + 1);
                } else {
                    break;
                }
            }

            expr = Expr::OpBinary(op, Box::new(expr), Box::new(rhs));
        }

        expr
    }

    fn parse_primary_expr(&mut self) -> Expr {
        let token = self.current_token.clone();
        
        match token {
            Token::Number(value) => {
                self.next_token(); // Consume the number
                Expr::Number(value)
            }
            Token::Identifier(ref id) => {
                self.next_token(); // Consume the identifier
                if self.current_token == Token::ParenL {
                    self.parse_function_call(id.clone())
                } else {
                    Expr::Variable(id.clone())
                }
            }
            Token::ParenL => {
                self.next_token(); // Consume left parenthesis
                let expr = self.parse_expression();
                self.expect_token(Token::ParenR);
                self.next_token(); // Consume right parenthesis
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_function_call(&mut self, name: String) -> Expr {
        self.expect_token(Token::ParenL);
        self.next_token(); // Consume left parenthesis

        let mut args = Vec::new();
        while self.current_token != Token::ParenR {
            let arg = self.parse_expression();
            args.push(arg);

            if self.current_token == Token::Comma {
                self.next_token(); // Consume comma
            } else {
                break;
            }
        }

        self.expect_token(Token::ParenR);
        self.next_token(); // Consume right parenthesis

        Expr::FunctionCall(name, args)
    }

    fn current_op(&self, precedence: u8) -> Option<Operation> {
        match self.current_token {
            Token::Plus if precedence <= 1 => Some(Operation::Add),
            Token::Minus if precedence <= 1 => Some(Operation::Sub),
            Token::Star if precedence <= 2 => Some(Operation::Mul),
            Token::Slash if precedence <= 2 => Some(Operation::Div),
            _ => None,
        }
    }

    fn precedence(&self, op: Operation) -> u8 {
        match op {
            Operation::Add | Operation::Sub => 1,
            Operation::Mul | Operation::Div => 2,
        }
    }

    pub fn parse(&mut self) -> Stmt {
        let mut stmts = Vec::new();

        while self.current_token != Token::Eof {
            match self.current_token {
                Token::Func => {
                    let func_stmt = self.parse_function_decl();
                    stmts.push(func_stmt);
                }
                Token::Var => {
                    let var_stmt = self.parse_variable_decl();
                    stmts.push(var_stmt);
                }
                _ => panic!("Unexpected token: {:?}", self.current_token),
            }
        }

        Stmt::Program(stmts)
    }
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
            Expr::Number(value) => write!(f, "{}", value),
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