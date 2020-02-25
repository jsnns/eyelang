pub enum TokenType {
    Symbol(String),
    Type(String),
    Number(i32),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Add,
    Proc,
    Main,
}

impl TokenType {
    pub fn tuple(&self) -> (&TokenType, String) {
        match self {
            TokenType::Symbol(value) => (self, value.to_string()),
            TokenType::Type(value) => (self, value.to_string()),
            TokenType::Number(value) => (self, value.to_string()),
            _ => (self, "".to_string()),
        }
    }
}

impl std::string::ToString for TokenType {
    fn to_string(&self) -> String {
        (match self {
            TokenType::Symbol(_) => "Symbol",
            TokenType::Number(_) => "Number",
            TokenType::Semicolon => ";",
            TokenType::Add => "Add",
            TokenType::Main => "Main",
            TokenType::Proc => "Proc",
            _ => "Not Defined",
        })
        .to_string()
    }
}

impl std::fmt::Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.tuple().1;
        write!(f, "({}, {})", self.to_string(), value)
    }
}
