use crate::types::ast::AST;

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
