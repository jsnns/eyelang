#[derive(Copy, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Assign,
}

impl BinaryOperator {
    pub fn get_precedence(&self) -> u8 {
        match self {
            BinaryOperator::Assign => 0,
            BinaryOperator::Add | BinaryOperator::Subtract => 10,
            BinaryOperator::Multiply | BinaryOperator::Divide => 20,
        }
    }
}

impl std::string::ToString for BinaryOperator {
    fn to_string(&self) -> String {
        (match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Assign => "=",
        })
        .to_string()
    }
}
