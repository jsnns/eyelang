use crate::ast::BinaryOperator;
use crate::ast::AST;
use crate::token::Token;
use std::cell::Cell;

// Implement LL(2) parser
pub fn get_precedence(operator: &BinaryOperator) -> u8 {
    match operator {
        BinaryOperator::Add | BinaryOperator::Subtract => 10,
        BinaryOperator::Multiply | BinaryOperator::Divide => 20,
    }
}

struct ParseState {
    tokens: Vec<Token>,
    curr_index: Cell<usize>,
}

impl ParseState {
    fn next(&self) -> &Token {
        self.curr_index.set(self.curr_index.get() + 1);
        self.current()
    }

    fn has_next(&self) -> bool {
        self.curr_index.get() + 1 < self.tokens.len()
    }

    fn current(&self) -> &Token {
        &self.tokens[self.curr_index.get()]
    }

    fn is_tok(&self, token: &Token) -> bool {
        if self.has_next() {
            return self.current() == token;
        }

        false
    }

    fn skip(&self, token: &Token) {
        if self.is_tok(token) {
            self.next();
        }
    }

    fn is_op(&self) -> bool {
        match self.current() {
            Token::Operator(BinaryOperator::Add)
            | Token::Operator(BinaryOperator::Subtract)
            | Token::Operator(BinaryOperator::Multiply)
            | Token::Operator(BinaryOperator::Divide) => true,
            _ => false,
        }
    }

    fn maybe_binary(&self, left: AST, precedence: u8) -> AST {
        if self.is_op() {
            if let Token::Operator(operator_token) = self.current() {
                let new_precedence = get_precedence(operator_token);
                if new_precedence > precedence {
                    self.next();
                    let right = self.maybe_binary(self.parse_atom(), new_precedence);
                    return self.maybe_binary(
                        AST::Binary {
                            operator: *operator_token,
                            left: Box::from(left),
                            right: Box::from(right),
                        },
                        precedence,
                    );
                }
            }
        }
        return left;
    }

    fn parse_atom(&self) -> AST {
        if !self.has_next() {
            return AST::EOF;
        }

        return self.maybe_binary(
            match self.current() {
                Token::Proc => self.parse_proc(),
                Token::Return => {
                    self.next();
                    AST::Return {
                        value: Box::from(self.parse_atom()),
                    }
                }
                Token::Number(val) => {
                    self.next();
                    AST::Number { value: *val }
                }
                Token::Bool(val) => {
                    self.next();
                    AST::Bool { value: *val }
                }
                Token::Semicolon => {
                    self.next();
                    return AST::Semicolon;
                }
                Token::Symbol(symbol) => {
                    self.next();
                    if self.is_tok(&Token::LParen) {
                        self.next();
                        return self.maybe_binary(self.parse_call(symbol.to_string()), 0);
                    } else {
                        AST::Assign {
                            symbol: symbol.to_string(),
                            value: Box::from(self.parse_atom()),
                        }
                    }
                }
                Token::Print => {
                    self.next();
                    AST::Print {
                        value: Box::from(self.parse_atom()),
                    }
                }
                _ => panic!(
                    "parser::parse_atom unimplemented for {}",
                    self.current().to_string()
                ),
            },
            0,
        );
    }

    fn parse_call(&self, symbol: String) -> AST {
        self.skip(&Token::RParen);
        AST::Call {
            func: symbol,
            args: vec![],
        }
    }

    fn parse_proc(&self) -> AST {
        // check the next token is a symbol
        if let Token::Symbol(proc_name) = self.next() {
            self.next();
            AST::Proc {
                symbol: proc_name.to_string(),
                value: self.parse_proc_body(),
            }
        } else {
            panic!(
                "Cannot create function without name {}",
                self.current().to_string()
            )
        }
    }

    fn parse_proc_body(&self) -> Vec<Box<AST>> {
        // TODO: handle args
        self.skip(&Token::LParen);
        self.skip(&Token::RParen);

        let mut proc_body: Vec<Box<AST>> = vec![];

        if *self.current() == Token::LBrace {
            self.next();
            while *self.current() != Token::RBrace {
                proc_body.push(Box::from(self.parse_atom()));
            }
            // skip past '}'
            self.skip(&Token::RBrace);
        } else {
            panic!("Expecting {{ found {:?}", self.current());
        }

        return proc_body;
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> AST {
    // println!("Tokens: {:?} \n", tokens);
    let mut prog: Vec<Box<AST>> = vec![];
    let parse_state = ParseState {
        tokens: tokens,
        curr_index: Cell::from(0),
    };

    while parse_state.has_next() {
        prog.push(Box::from(parse_state.parse_atom()));
        parse_state.skip(&Token::Semicolon);
    }

    let prog = AST::Program { program: prog };

    // println!("{:?}", prog);

    return prog;
}
