mod lexer;
pub use self::lexer::tokenize;

#[cfg(test)]
mod tokens {
    use super::*;
    use crate::types::binary_operator::BinaryOperator;
    use crate::types::token::Token;
    #[test]
    fn neg_numbers() {
        let program = "-1;".to_string();
        let tokens = lexer::tokenize(program).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Operator(BinaryOperator::Subtract),
                Token::Number(1),
                Token::Semicolon
            ]
        )
    }
    #[test]
    fn add_numbers() {
        let program = "10 + 20 * 1;".to_string();
        let tokens = lexer::tokenize(program).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(10),
                Token::Operator(BinaryOperator::Add),
                Token::Number(20),
                Token::Operator(BinaryOperator::Multiply),
                Token::Number(1),
                Token::Semicolon
            ]
        )
    }

    #[test]
    fn def_proc() {
        let program = "define a to be {return false;}".to_string();
        let tokens = lexer::tokenize(program).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Define,
                Token::Symbol("a".to_string()),
                Token::ToBe,
                Token::LBrace,
                Token::Return,
                Token::Bool(false),
                Token::Semicolon,
                Token::RBrace,
            ]
        )
    }
}
