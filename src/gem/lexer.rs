#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(f64),
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
            ($self:ident, $token:expr) => {{
                $self.pos += 1;
                $token
            }};
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
        Token::Number(number)
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
            _ => Token::Identifier(identifier),
        }
    }
}
