use crate::common::{Stmt, Expr, Operation, Token};
use crate::lexer::*;

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
            panic!("Expected token: {:?}, got: {:?}", token, self.current_token);
        }
    }

    #[inline(always)]
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

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

        let body = self.parse_block();

        self.consume(Token::BraceR);

        Stmt::FunctionDecl(Expr::Null, name, params, Box::new(body))
    }

    fn parse_variable_decl(&mut self) -> Stmt {
        self.consume(Token::Var);
    
        let name = if let Token::Identifier(ref id) = self.current_token {
            id.clone()
        } else {
            panic!("Expected variable name");
        };
    
        self.next_token();
    
        let mut value = None;
    
        if self.current_token == Token::Eq {
            self.next_token();
            value = Some(self.parse_expression());
        }
    
        self.consume(Token::Semicolon);
    
        Stmt::VariableDecl(name, value.unwrap_or(Expr::LitNum(0.0)))
    }

    fn parse_expression(&mut self) -> Expr {
        match self.current_token {
            Token::LitNum(_) | Token::LitStr(_) | Token::Identifier(_) | Token::ParenL => self.parse_binop_expr(0),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.current_token {
            Token::Var => self.parse_variable_decl(),
            Token::Func => self.parse_function_decl(),
            _ => Stmt::Expr(self.parse_expression()),
        }
    }

    fn parse_block(&mut self) -> Stmt {
        let mut stmts = Vec::new();

        while self.current_token != Token::BraceR && self.current_token != Token::Eof {
            stmts.push(self.parse_statement());
            if self.current_token == Token::Semicolon {
                self.next_token(); // Skip semicolons separating statements
            }
        }

        Stmt::Program(stmts)
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
            Token::LitNum(value) => {
                self.next_token();
                Expr::LitNum(value)
            }
            Token::LitStr(ref value) => {
                self.next_token();
                Expr::LitStr(value.clone())
            }
            Token::Identifier(ref id) => {
                self.next_token();
                if self.current_token == Token::ParenL {
                    self.parse_function_call_expr(id.clone())
                } else {
                    Expr::Variable(id.clone())
                }
            }
            Token::ParenL => {
                self.next_token();
                let expr = self.parse_expression();
                self.consume(Token::ParenR);
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_function_call_expr(&mut self, name: String) -> Expr {
        self.consume(Token::ParenL);

        let mut args = Vec::new();
        while self.current_token != Token::ParenR {
            let arg = self.parse_expression();
            args.push(arg);

            if self.current_token == Token::Comma {
                self.next_token();
            } else {
                break;
            }
        }

        self.consume(Token::ParenR);

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
            stmts.push(self.parse_statement());
        }

        Stmt::Program(stmts)
    }
}
