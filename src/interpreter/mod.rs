mod interpreter;
pub use self::interpreter::interpret;

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::ast::AST;
    use crate::types::binary_operator::BinaryOperator;
    use crate::types::options::Options;
    use crate::types::primitive_value::PrimitiveValue;
    use std::collections::HashMap;

    #[test]
    fn run_operator_on_numbers() {
        // we have to return the value to read it
        let program = vec![Box::from(AST::Return {
            value: Box::from(AST::Binary {
                operator: BinaryOperator::Add,
                right: Box::from(AST::Binary {
                    operator: BinaryOperator::Multiply,
                    left: Box::from(AST::Number { value: 20 }),
                    right: Box::from(AST::Number { value: 2 }),
                }),
                left: Box::from(AST::Number { value: 10 }),
            }),
        })];

        let mut symbols = HashMap::new();
        let result = interpreter::run_body_and_return(program, &mut symbols, &Options::debug())
            .unwrap()
            .unwrap();

        if let PrimitiveValue::Num(result) = result {
            assert_eq!(result, 50);
        }
    }
}
