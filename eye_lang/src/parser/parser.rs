use crate::types::ast::If;
use crate::types::ast::AST;
use crate::types::binary_operator::BinaryOperator;
use crate::types::token::Token;
use std::cell::Cell;

pub fn build_program(tokens: Vec<Token>) -> AST {
    println!("Tokens: {:?}", tokens);
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

    return prog;
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
            | Token::Operator(BinaryOperator::Divide)
            | Token::Operator(BinaryOperator::Assign)
            | Token::Operator(BinaryOperator::IsEq) => true,
            _ => false,
        }
    }

    fn maybe_binary(&self, left: AST, precedence: u8) -> AST {
        if self.is_op() {
            if let Token::Operator(operator_token) = self.current() {
                let new_precedence = operator_token.get_precedence();
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
            } else {
                panic!("Could not get operator.")
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
                Token::Str(value) => {
                    self.next();
                    AST::Str {
                        value: value.to_string(),
                    }
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
                        AST::Symbol {
                            identifier: symbol.to_string(),
                        }
                    }
                }
                Token::Print => {
                    self.next();
                    AST::Print {
                        value: Box::from(self.parse_atom()),
                    }
                }
                Token::Set => {
                    self.next();
                    self.parse_set()
                }
                Token::If => {
                    self.next();
                    self.parse_if()
                }
                Token::Do => {
                    self.next();
                    self.parse_do()
                }
                _ => panic!(
                    "parser::parse_atom unimplemented for {}",
                    self.current().to_string()
                ),
            },
            0,
        );
    }

    fn parse_do(&self) -> AST {
        let count = self.parse_atom();
        if let Token::Symbol(identifier) = self.current() {
            self.next();
            AST::Do {
                count: Box::from(count),
                identifier: identifier.clone(),
                body: self.parse_proc_body(),
            }
        } else {
            panic!(
                "Do loops must have an index symbol found: {:?}",
                self.current()
            );
        }
    }

    fn parse_if(&self) -> AST {
        AST::If {
            this: If {
                conditional: Box::from(self.parse_atom()),
                body: self.parse_proc_body(),
            },
            elifs: self.parse_elif(),
            el: self.parse_el(),
        }
    }

    fn parse_elif(&self) -> Option<Vec<If>> {
        let mut elifs: Vec<If> = vec![];
        if self.is_tok(&Token::Else) {
            while self.is_tok(&Token::Else) {
                self.skip(&Token::Else);
                // else after if-else
                if self.is_tok(&Token::LBrace) {
                    return Some(elifs);
                }
                assert_eq!(*self.current(), Token::If);
                self.next();
                elifs.push(If {
                    conditional: Box::from(self.parse_atom()),
                    body: self.parse_proc_body(),
                })
            }
        } else {
            return None;
        }

        Some(elifs)
    }

    fn parse_el(&self) -> Option<Vec<Box<AST>>> {
        if self.is_tok(&Token::Else) {
            self.skip(&Token::Else);
            Some(self.parse_proc_body())
        } else {
            None
        }
    }

    fn parse_set(&self) -> AST {
        if let Token::Symbol(symbol) = self.current() {
            self.next();
            self.skip(&Token::Operator(BinaryOperator::Assign));
            AST::Assign {
                identifier: symbol.to_string(),
                value: Box::from(self.parse_atom()),
            }
        } else {
            panic!(
                "Can't determine symbol name for set at {:?}",
                self.current()
            );
        }
    }

    fn parse_call(&self, symbol: String) -> AST {
        self.skip(&Token::LParen);
        AST::Call {
            identifier: symbol,
            args: self.parse_call_args(),
        }
    }

    fn parse_call_args(&self) -> Vec<Box<AST>> {
        let mut asts: Vec<Box<AST>> = vec![];

        while !self.is_tok(&Token::RParen) {
            asts.push(Box::from(self.parse_atom()));
            self.skip(&Token::Comma);
        }

        self.skip(&Token::RParen);

        return asts;
    }

    fn parse_func_args(&self) -> Vec<String> {
        self.skip(&Token::LParen);
        let mut tokens = vec![];

        loop {
            if let Token::Symbol(symbol) = self.current() {
                self.next();
                tokens.push(symbol.to_string());
                self.skip(&Token::Comma);
            } else {
                break;
            }
        }

        self.skip(&Token::RParen);
        return tokens;
    }

    fn parse_proc(&self) -> AST {
        // check the next token is a symbol
        if let Token::Symbol(proc_name) = self.next() {
            self.next();
            AST::Proc {
                identifier: proc_name.to_string(),
                args: self.parse_func_args(),
                body: self.parse_proc_body(),
            }
        } else {
            panic!(
                "Cannot create function without name {}",
                self.current().to_string()
            )
        }
    }

    fn parse_proc_body(&self) -> Vec<Box<AST>> {
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
