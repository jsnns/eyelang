#[allow(dead_code)]
#[derive(Clone)]
pub enum AST {
    Binary {
        operator: BinaryOperator,
        left: Box<AST>,
        right: Box<AST>,
    },
    Call,
    Assign,
    Num {
        value: i32,
    },
    String,
    Bool,
    Let,
    If,
    Program {
        program: Vec<Box<AST>>,
    },
}

impl std::string::ToString for AST {
    fn to_string(&self) -> String {
        match self {
            AST::Binary {
                operator,
                left,
                right,
            } => format!("{:?} {:?}, {}", left, right, operator.to_string()),
            AST::Num { value } => format!("{}", value),
            AST::Program { program } => format!("Program: {:?}", program),
            _ => format!(""),
        }
    }
}

impl std::fmt::Debug for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Copy, Clone)]
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
#[derive(Debug)]
pub enum PrimitiveValue {
    String(String),
    Num(i32),
    Bool(bool),
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
        }
    }
}
