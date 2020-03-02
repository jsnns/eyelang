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
            _ => format!("N/A"),
        }
    }
}

impl std::fmt::Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl std::string::ToString for BinaryOperator {
    fn to_string(&self) -> String {
        (match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
        })
        .to_string()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PrimitiveValue {
    String(String),
    Num(i32),
    Bool(bool),
    Block(Vec<AST>),
}

#[derive(Debug, Clone)]
pub struct NotImplemented {
    a: String,
    b: String,
}
impl NotImplemented {
    pub fn from(a: &PrimitiveValue, b: &PrimitiveValue) -> NotImplemented {
        NotImplemented {
            a: a.to_string(),
            b: b.to_string(),
        }
    }
}
type OperatorValue<T> = Result<T, NotImplemented>;

impl PrimitiveValue {
    pub fn add(self: Self, other: Self) -> OperatorValue<Self> {
        let err_val = NotImplemented::from(&self, &other);
        match self {
            PrimitiveValue::String(a) => match other {
                PrimitiveValue::String(b) => {
                    Ok(PrimitiveValue::String(a.to_string() + &b.to_string()))
                }
                _ => Err(err_val),
            },
            PrimitiveValue::Num(a) => match other {
                PrimitiveValue::Num(b) => Ok(PrimitiveValue::Num(a + b)),
                _ => Err(err_val),
            },
            _ => Err(err_val),
        }
    }

    pub fn subtract(self, other: Self) -> OperatorValue<Self> {
        let err_val = NotImplemented::from(&self, &other);
        match self {
            PrimitiveValue::Num(a) => match other {
                PrimitiveValue::Num(b) => Ok(PrimitiveValue::Num(a - b)),
                _ => Err(err_val),
            },
            _ => Err(err_val),
        }
    }

    pub fn multiply(self, other: Self) -> OperatorValue<Self> {
        let err_val = NotImplemented::from(&self, &other);
        match self {
            PrimitiveValue::Num(a) => match other {
                PrimitiveValue::Num(b) => Ok(PrimitiveValue::Num(a * b)),
                _ => Err(err_val),
            },
            _ => Err(err_val),
        }
    }

    pub fn get_operator_value(operator_value: OperatorValue<Self>) -> Self {
        if let Ok(value) = operator_value {
            value
        } else {
            panic!("{} {}")
        }
    }
}

impl std::string::ToString for PrimitiveValue {
    fn to_string(&self) -> String {
        match self {
            PrimitiveValue::Bool(val) => val.to_string(),
            PrimitiveValue::String(val) => val.to_string(),
            PrimitiveValue::Num(val) => val.to_string(),
            PrimitiveValue::Block(body) => format!("{:?}", body),
        }
    }
}
