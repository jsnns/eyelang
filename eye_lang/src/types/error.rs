use crate::types::primitive_type::PrimitiveValue;

#[derive(Debug)]
pub struct TokenError;

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

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl std::convert::From<NotImplemented> for RuntimeError {
    fn from(a: NotImplemented) -> RuntimeError {
        RuntimeError {
            message: format!("Operator not implemented for {} and {}.", a.a, a.b),
        }
    }
}
