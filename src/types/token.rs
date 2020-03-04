use crate::types::binary_operator::BinaryOperator;

#[derive(Clone, PartialEq)]
pub enum Token {
    Symbol(String),
    Type(String),
    Str(String),
    Number(i32),
    Bool(bool),
    Operator(BinaryOperator),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Proc,
    Comma,
    Return,
    Print,
    Set,
    If,
    Else,
    Do,
    Throw,
}

impl Token {
    pub fn tuple(&self) -> (&Token, String) {
        match self {
            Token::Symbol(value) => (self, value.to_string()),
            Token::Type(value) => (self, value.to_string()),
            Token::Number(value) => (self, value.to_string()),
            Token::Operator(value) => (self, value.to_string()),
            Token::Bool(value) => (self, value.to_string()),
            Token::Str(value) => (self, value.to_string()),
            _ => (self, "".to_string()),
        }
    }
}

impl std::string::ToString for Token {
    fn to_string(&self) -> String {
        let str: &str = match self {
            Token::Symbol(_) => "Symbol",
            Token::Bool(_) => "Bool",
            Token::Number(_) => "Number",
            Token::Type(_) => "Type",
            Token::Str(_) => "Str",
            Token::Operator(..) => "Operator",
            Token::Proc => "Proc",
            Token::Return => "Return",
            Token::Comma => "Comma",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::Semicolon => ";",
            Token::Print => "Print",
            Token::Set => "Set",
            Token::If => "If",
            Token::Else => "Else",
            Token::Do => "Do",
            Token::Throw => "Throw",
        };

        str.to_string()
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.tuple().1;
        write!(f, "({}, {})", self.to_string(), value)
    }
}
