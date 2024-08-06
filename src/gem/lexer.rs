use crate::common::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        while self.pos < self.input.len() && self.current_char().unwrap().is_whitespace() {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let c = self.current_char().unwrap();

        macro_rules! opdef {
            ($self:ident, $token:expr) => {{ $self.pos += 1; $token }};
        }

        match c {
            '+' => opdef!(self, Token::Plus),
            '-' => opdef!(self, Token::Minus),
            '*' => opdef!(self, Token::Star),
            '/' => opdef!(self, Token::Slash),
            '=' => opdef!(self, Token::Eq),
            ':' => opdef!(self, Token::Colon),
            ',' => opdef!(self, Token::Comma),
            '.' => opdef!(self, Token::Dot),
            ';' => opdef!(self, Token::Semicolon),
            '(' => opdef!(self, Token::ParenL),
            ')' => opdef!(self, Token::ParenR),
            '{' => opdef!(self, Token::BraceL),
            '}' => opdef!(self, Token::BraceR),
            '"' => self.parse_string_literal(),
            '0'..='9' => self.parse_number(),
            'a'..='z' | 'A'..='Z' => self.parse_identifier(),
            _ => {
                self.pos += 1;
                self.next_token()
            }
        }
    }
    
    #[inline(always)]
    fn current_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn parse_number(&mut self) -> Token {
        let start = self.pos;

        while self.pos < self.input.len() && self.current_char().unwrap().is_digit(10) {
            self.pos += 1;
        }

        if self.pos < self.input.len() && self.current_char() == Some('.') {
            self.pos += 1;
            while self.pos < self.input.len() && self.current_char().unwrap().is_digit(10) {
                self.pos += 1;
            }
        }

        let number: f64 = self.input[start..self.pos].parse().unwrap_or(0.0);
        Token::LitNum(number)
    }

    fn parse_identifier(&mut self) -> Token {
        let start = self.pos;

        while self.pos < self.input.len() && self.current_char().unwrap().is_alphanumeric() {
            self.pos += 1;
        }

        let identifier = self.input[start..self.pos].to_string();
        match identifier.as_str() {
            "func" => Token::Func,
            "var" => Token::Var,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "return" => Token::Return,
            _ => Token::Identifier(identifier),
        }
    }

    fn parse_string_literal(&mut self) -> Token {
        self.pos += 1; // Skip the opening quote
    
        let mut result = String::new();

        while self.pos < self.input.len() {
            match self.current_char() {
                Some('"') => {
                    self.pos += 1;
                    return Token::LitStr(result);
                }
                Some('\\') => {
                    // Handling escape sequences
                    self.pos += 1;
                    if self.pos < self.input.len() {
                        match self.current_char() {
                            Some('n') => result.push('\n'),
                            Some('t') => result.push('\t'),
                            Some('"') => result.push('"'),
                            Some('\\') => result.push('\\'),
                            _ => {
                                result.push('\\');
                                result.push(self.current_char().unwrap());
                            }
                        }
                        self.pos += 1;
                    }
                }
                Some(c) => result.push(c),
                None => {
                    return Token::LitStr(result);
                }
            }
            self.pos += 1;
        }
    
        Token::LitStr(result)
    }
}