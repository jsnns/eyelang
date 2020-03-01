use crate::ast::AST;
use crate::token::Token;

// Implement LL(2) parser

// returns the new AST component and
// the number of tokens used for the AST
pub fn ast_from_three(a: &Token, b: &Token, c: &Token) -> (AST, usize) {
    match (a, b, c) {
        (&Token::Number(num_a), Token::Operator(operator), &Token::Number(num_b)) => (
            AST::Binary {
                operator: *operator,
                left: Box::from(AST::Num { value: num_a }),
                right: Box::from(AST::Num { value: num_b }),
            },
            3,
        ),
        _ => panic!("Could not match tokens to AST, {:?}, {:?}, {:?}", a, b, c),
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> AST {
    let mut program: Vec<Box<AST>> = vec![];

    let mut i = 0;
    while i < tokens.len() {
        if i + 2 < tokens.len() {
            let (new_ast, tokens_used) = ast_from_three(&tokens[i], &tokens[i + 1], &tokens[i + 2]);
            i += tokens_used;
            program.push(Box::from(new_ast));
        }
    }

    println!("{:?}", program);

    AST::Program { program: program }
}
