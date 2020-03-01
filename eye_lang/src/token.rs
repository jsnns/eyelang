use crate::ast::BinaryOperator;

pub enum Token {
    Symbol(String),
    Type(String),
    Number(i32),
    Operator(BinaryOperator),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Proc,
    Main,
    Comma,
    Return,
}

impl Token {
    pub fn tuple(&self) -> (&Token, String) {
        match self {
            Token::Symbol(value) => (self, value.to_string()),
            Token::Type(value) => (self, value.to_string()),
            Token::Number(value) => (self, value.to_string()),
            Token::Operator(value) => (self, value.to_string()),
            _ => (self, "".to_string()),
        }
    }
}

impl std::string::ToString for Token {
    fn to_string(&self) -> String {
        (match self {
            Token::Symbol(_) => "Symbol",
            Token::Number(_) => "Number",
            Token::Type(_) => "Type",
            Token::Operator(_) => "Operator",
            Token::Main => "Main",
            Token::Proc => "Proc",
            Token::Return => "Return",
            Token::Comma => "Comma",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::Semicolon => ";",
        })
        .to_string()
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.tuple().1;
        write!(f, "({}, {})", self.to_string(), value)
    }
}
