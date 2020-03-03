use crate::types::binary_operator::BinaryOperator;

#[allow(dead_code)]
#[derive(Clone)]
pub enum AST {
    Binary {
        operator: BinaryOperator,
        left: Box<AST>,
        right: Box<AST>,
    },
    Call {
        func: String,
        args: Vec<Box<AST>>,
    },
    Assign {
        symbol: String,
        value: Box<AST>,
    },
    Proc {
        symbol: String,
        value: Vec<Box<AST>>,
    },
    Return {
        value: Box<AST>,
    },
    Number {
        value: i32,
    },
    String {
        value: String,
    },
    Bool {
        value: bool,
    },
    If,
    Program {
        program: Vec<Box<AST>>,
    },
    EOF,
    Semicolon,
    Print {
        value: Box<AST>,
    },
    Symbol {
        name: String,
    },
}

impl std::string::ToString for AST {
    fn to_string(&self) -> String {
        match self {
            AST::Binary {
                operator,
                left,
                right,
            } => format!("{:?} {} {:?}", left, operator.to_string(), right),
            AST::Number { value } => format!("{}", value),
            AST::Program { program } => format!("Program: {:?}", program),
            AST::Call { func, args } => format!("Call {}({:?})", func, args),
            AST::Proc { symbol, value } => format!("Proc {} {:?}", symbol, value),
            AST::Return { value } => format!("Return <{:?}>", value),
            AST::Bool { value } => format!("Bool {}", value),
            AST::Print { value } => format!("Print {:?}", value),
            AST::Semicolon => format!(";"),
            AST::Assign { symbol, value } => format!("Assign {} = {:?}", symbol, value),
            AST::Symbol { name } => format!("Symbol ({})", name),
            _ => format!("N/A"),
        }
    }
}

impl std::fmt::Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
