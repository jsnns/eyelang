use crate::types::body::Block;
use crate::types::error::NotImplemented;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PrimitiveValue {
    Str(String),
    Num(i32),
    Bool(bool),
    Block(Block),
}

type OperatorValue<T> = Result<T, NotImplemented>;

impl PrimitiveValue {
    pub fn add(self: Self, other: Self) -> OperatorValue<Self> {
        let err_val = NotImplemented::from(&self, &other);
        match self {
            PrimitiveValue::Str(a) => match other {
                PrimitiveValue::Str(b) => Ok(PrimitiveValue::Str(format!("{}{}", a, b))),
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

    pub fn is_equal(self, other: Self) -> OperatorValue<Self> {
        let err_val = NotImplemented::from(&self, &other);
        match self {
            PrimitiveValue::Num(a) => match other {
                PrimitiveValue::Num(b) => Ok(PrimitiveValue::Bool(a == b)),
                _ => Err(err_val),
            },
            PrimitiveValue::Bool(a) => match other {
                PrimitiveValue::Bool(b) => Ok(PrimitiveValue::Bool(a == b)),
                _ => Err(err_val),
            },
            PrimitiveValue::Str(a) => match other {
                PrimitiveValue::Str(b) => Ok(PrimitiveValue::Bool(a == b)),
                _ => Err(err_val),
            },
            _ => Err(err_val),
        }
    }
}

impl std::string::ToString for PrimitiveValue {
    fn to_string(&self) -> String {
        match self {
            PrimitiveValue::Bool(val) => val.to_string(),
            PrimitiveValue::Str(val) => val.to_string(),
            PrimitiveValue::Num(val) => val.to_string(),
            PrimitiveValue::Block(block) => format!("({:?}):{{{:?}}}", block.args, block.body),
        }
    }
}
