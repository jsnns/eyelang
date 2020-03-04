use crate::types::binary_operator::BinaryOperator;
use crate::types::symbol_store::Identifier;

pub type Block = Vec<Box<AST>>;

#[derive(Debug, Clone)]
pub struct FunctionBody {
    pub body: Block,
    pub args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If {
    pub conditional: Box<AST>,
    pub body: Block,
}

#[derive(Clone, PartialEq, Eq)]
pub enum AST {
    Symbol {
        identifier: Identifier,
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
        identifier: Identifier,
        value: Box<AST>,
    },
    Proc {
        identifier: Identifier,
        args: Vec<String>,
        body: Block,
    },
    Call {
        identifier: Identifier,
        args: Block,
    },
    Return {
        value: Box<AST>,
    },
    If {
        this: If,
        elifs: Option<Vec<If>>,
        el: Option<Block>,
    },
    Print {
        value: Box<AST>,
    },
    Program {
        program: Block,
    },
    Do {
        count: Box<AST>,
        identifier: Option<Identifier>,
        body: Block,
    },
    Throw {
        message: String,
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
            AST::Symbol { identifier } => format!("Symbol ({})", identifier),
            // ctrl characters
            AST::Semicolon => format!(";"),
            AST::EOF => format!("EOF"),
            // actions
            AST::Assign { identifier, value } => format!("Assign {} = {:?}", identifier, value),
            AST::Print { value } => format!("Print {:?}", value),
            AST::Throw { message } => format!("Throw {}", message),
            AST::Binary {
                operator,
                left,
                right,
            } => format!("Binary {:?} {} {:?}", left, operator.to_string(), right),
            // blocked calls
            AST::Program { program } => format!("Program: {:?}", program),
            AST::Call { identifier, args } => format!("Call {}({:?})", identifier, args),
            AST::If { this, elifs, el } => format!("If {:?} {:?} {:?}", this, elifs, el),
            AST::Return { value } => format!("Return <{:?}>", value),
            AST::Proc {
                identifier,
                body,
                args,
            } => format!("Proc {} {:?}({:?})", identifier, body, args),
            AST::Do {
                count,
                identifier,
                body,
            } => format!("Do {:?}:{:?} {:?}", count, identifier, body),
        }
    }
}

impl std::fmt::Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
