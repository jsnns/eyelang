use crate::types::binary_operator::BinaryOperator;

type Block = Vec<Box<AST>>;

#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub body: Vec<Box<AST>>,
    pub args: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct If {
    pub conditional: Box<AST>,
    pub body: Vec<Box<AST>>,
}

#[derive(Clone)]
pub enum AST {
    Symbol {
        name: String,
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
    Binary {
        operator: BinaryOperator,
        left: Box<AST>,
        right: Box<AST>,
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
    Call {
        func: String,
        args: Vec<Box<AST>>,
    },
    Return {
        value: Box<AST>,
    },
    If {
        this: If,
        elifs: Option<Vec<If>>,
        el: Option<Vec<Box<AST>>>,
    },
    Print {
        value: Box<AST>,
    },
    Program {
        program: Vec<Box<AST>>,
    },
    EOF,
    Semicolon,
}

impl std::string::ToString for AST {
    fn to_string(&self) -> String {
        match self {
            // literals
            AST::Number { value } => format!("Number({})", value),
            AST::Str { value } => format!("String({})", value),
            AST::Bool { value } => format!("Bool({})", value),
            AST::Symbol { name } => format!("Symbol ({})", name),
            // ctrl characters
            AST::Semicolon => format!(";"),
            AST::EOF => format!("EOF"),
            // actions
            AST::Assign { symbol, value } => format!("Assign {} = {:?}", symbol, value),
            AST::Print { value } => format!("Print {:?}", value),
            AST::Binary {
                operator,
                left,
                right,
            } => format!("Binary {:?} {} {:?}", left, operator.to_string(), right),
            // blocked calls
            AST::Program { program } => format!("Program: {:?}", program),
            AST::Call { func, args } => format!("Call {}({:?})", func, args),
            AST::If { this, elifs, el } => format!("If {:?} {:?} {:?}", this, elifs, el),
            AST::Return { value } => format!("Return <{:?}>", value),
            AST::Proc {
                symbol,
                value,
                args,
            } => format!("Proc {} {:?}({:?})", symbol, value, args),
        }
    }
}

impl std::fmt::Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
