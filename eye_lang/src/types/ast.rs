use crate::types::binary_operator::BinaryOperator;

#[derive(Clone, Debug)]
pub struct If {
    pub conditional: Box<AST>,
    pub body: Vec<Box<AST>>,
}

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
        args: Vec<String>,
        value: Vec<Box<AST>>,
    },
    Return {
        value: Box<AST>,
    },
    Number {
        value: i32,
    },
    Str {
        value: String,
    },
    Bool {
        value: bool,
    },
    If {
        this: If,
        elifs: Option<Vec<If>>,
        el: Option<Vec<Box<AST>>>,
    },
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
            } => format!("Binary {:?} {} {:?}", left, operator.to_string(), right),
            AST::Number { value } => format!("Number({})", value),
            AST::Str { value } => format!("String({})", value),
            AST::Bool { value } => format!("Bool({})", value),
            AST::Program { program } => format!("Program: {:?}", program),
            AST::Call { func, args } => format!("Call {}({:?})", func, args),
            AST::Proc {
                symbol,
                value,
                args,
            } => format!("Proc {} {:?}({:?})", symbol, value, args),
            AST::Return { value } => format!("Return <{:?}>", value),
            AST::Print { value } => format!("Print {:?}", value),
            AST::Semicolon => format!(";"),
            AST::Assign { symbol, value } => format!("Assign {} = {:?}", symbol, value),
            AST::Symbol { name } => format!("Symbol ({})", name),
            AST::If { this, elifs, el } => format!("If {:?} {:?} {:?}", this, elifs, el),
            AST::EOF => format!("EOF"),
        }
    }
}

impl std::fmt::Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
