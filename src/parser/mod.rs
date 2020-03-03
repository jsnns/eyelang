mod parser;
pub use self::parser::build_program;

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::ast::AST;
    use crate::types::binary_operator::BinaryOperator;
    use crate::types::token::Token;

    #[test]
    fn add_numbers() {
        let tokens = vec![
            Token::Number(10),
            Token::Operator(BinaryOperator::Add),
            Token::Number(20),
            Token::Operator(BinaryOperator::Multiply),
            Token::Number(1),
            Token::Semicolon,
        ];
        let ast = parser::build_program(tokens);

        assert_eq!(
            ast,
            AST::Program {
                program: vec![Box::from(AST::Binary {
                    operator: BinaryOperator::Add,
                    right: Box::from(AST::Binary {
                        operator: BinaryOperator::Multiply,
                        left: Box::from(AST::Number { value: 20 }),
                        right: Box::from(AST::Number { value: 1 })
                    }),
                    left: Box::from(AST::Number { value: 10 })
                })]
            }
        )
    }

    #[test]
    fn print_is_eq() {
        let tokens = vec![
            Token::Print,
            Token::Number(10),
            Token::Operator(BinaryOperator::IsEq),
            Token::Number(20),
            Token::Semicolon,
        ];
        let ast = parser::build_program(tokens);

        assert_eq!(
            ast,
            AST::Program {
                program: vec![Box::from(AST::Print {
                    value: Box::from(AST::Binary {
                        operator: BinaryOperator::IsEq,
                        left: Box::from(AST::Number { value: 10 }),
                        right: Box::from(AST::Number { value: 20 })
                    })
                })]
            }
        )
    }
}
