use crate::types::ast::FunctionBody;
use crate::types::error::NotImplemented;

type OperatorValue<T> = Result<T, NotImplemented>;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveValue {
    Str(String),
    Num(i32),
    Bool(bool),
    Function(FunctionBody),
}

impl std::ops::Not for PrimitiveValue {
    type Output = PrimitiveValue;
    fn not(self) -> PrimitiveValue {
        if let PrimitiveValue::Bool(val) = self {
            PrimitiveValue::Bool(!val)
        } else {
            PrimitiveValue::Bool(false)
        }
    }
}

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

    #[allow(dead_code)]
    pub fn assert_false(&self) {
        PrimitiveValue::check(!self.clone());
    }

    #[allow(dead_code)]
    pub fn assert(&self) {
        PrimitiveValue::check(self.clone());
    }

    fn check(a: PrimitiveValue) {
        if let PrimitiveValue::Bool(result) = a {
            assert!(result);
        } else {
            panic!("Was not given a bool val.");
        }
    }
}

impl std::string::ToString for PrimitiveValue {
    fn to_string(&self) -> String {
        match self {
            PrimitiveValue::Bool(val) => val.to_string(),
            PrimitiveValue::Str(val) => val.to_string(),
            PrimitiveValue::Num(val) => val.to_string(),
            PrimitiveValue::Function(block) => format!("({:?}):{{{:?}}}", block.args, block.body),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_add() {
        // numbers
        let a = PrimitiveValue::Num(5);
        let b = PrimitiveValue::Num(2);
        let result = a.add(b).unwrap();
        if let PrimitiveValue::Num(val) = result {
            assert_eq!(val, 7);
        }

        // strings
        let a = PrimitiveValue::Str("abc".to_string());
        let b = PrimitiveValue::Str("def".to_string());
        let result = a.add(b).unwrap();
        if let PrimitiveValue::Str(val) = result {
            assert_eq!(val, "abcdef".to_string());
        }
    }

    #[test]
    fn can_not_add() {
        let a = PrimitiveValue::Bool(true);
        let b = PrimitiveValue::Bool(false);
        assert!(a.add(b).is_err());
    }

    #[test]
    fn can_multiply() {
        let a = PrimitiveValue::Num(2);
        let b = PrimitiveValue::Num(4);
        let result = a.multiply(b).unwrap();
        if let PrimitiveValue::Num(val) = result {
            assert_eq!(val, 8);
        }
    }

    #[test]
    fn can_not_multiply() {
        let a = PrimitiveValue::Num(2);
        let b = PrimitiveValue::Bool(true);
        assert!(a.multiply(b).is_err());
    }

    #[test]
    fn is_eq() {
        let a = PrimitiveValue::Num(2);
        let b = PrimitiveValue::Num(3);
        a.clone().is_equal(b.clone()).unwrap().assert_false();
        a.clone().is_equal(a.clone()).unwrap().assert();

        let a = PrimitiveValue::Str("a".to_string());
        let b = PrimitiveValue::Str("b".to_string());
        a.is_equal(b).unwrap().assert_false();

        let a = PrimitiveValue::Bool(true);
        let b = PrimitiveValue::Bool(false);
        a.is_equal(b).unwrap().assert_false();
    }

    #[test]
    fn not() {
        (!PrimitiveValue::Bool(false)).assert();
    }
}
