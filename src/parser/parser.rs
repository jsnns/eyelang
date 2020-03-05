use crate::types::ast::If;
use crate::types::ast::AST;
use crate::types::binary_operator::BinaryOperator;
use crate::types::symbol_store::Identifier;
use crate::types::token::Token;
use std::cell::Cell;

pub fn build_program(tokens: Vec<Token>) -> AST {
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
                // Token::Define => self.parse_proc(),
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
                Token::Define => {
                    self.next();
                    self.parse_define()
                    // self.parse_set()
                }
                Token::If => {
                    self.next();
                    self.parse_if()
                }
                Token::Do => {
                    self.next();
                    self.parse_do()
                }
                Token::Run => {
                    self.next();
                    self.parse_run()
                }
                Token::Throw => {
                    self.next();
                    if let Token::Str(message) = self.current() {
                        self.next();
                        AST::Throw {
                            message: message.to_string(),
                        }
                    } else {
                        panic!("Can't find message to throw!");
                    }
                }
                // handle negative numbers
                Token::Operator(operator) => {
                    if *operator == BinaryOperator::Subtract {
                        // skip operator
                        self.next();

                        if let Token::Number(num) = self.current() {
                            return AST::Number { value: -num };
                        }
                    }

                    panic!("Can't apply operator {}", operator.to_string());
                }
                _ => panic!(
                    "parser::parse_atom unimplemented for {}",
                    self.current().to_string()
                ),
            },
            0,
        );
    }

    fn parse_run(&self) -> AST {
        if let Token::Symbol(symbol) = self.current() {
            // skip symbol
            self.next();
            // skip given token
            let mut args: Vec<Box<AST>> = vec![];
            if self.is_tok(&Token::Given) {
                self.next();
                self.skip(&Token::LParen);
                args = self.parse_call_args();
            }
            AST::Call {
                identifier: symbol.to_string(),
                args: args,
            }
        } else {
            panic!(
                "Could not find function name to call found: {:?}",
                self.current()
            );
        }
    }

    fn parse_define(&self) -> AST {
        if let Token::Symbol(symbol) = self.current() {
            // skip past symbol
            self.next();

            // assert the next token is "to be"
            assert!(self.is_tok(&Token::ToBe));
            self.next();

            match self.current() {
                // fn if next char is {
                Token::LBrace => self.parse_proc(symbol),

                // othersie it's a var
                _ => self.parse_set(symbol),
            }
        } else {
            panic!("Could not get symbol for define found {:?}", self.current());
        }
    }

    fn parse_do(&self) -> AST {
        let count = self.parse_atom();
        let mut identifier_value: Option<Identifier> = None;
        if let Token::Symbol(identifier) = self.current() {
            self.next();
            identifier_value = Some(identifier.to_string());
        }
        AST::Do {
            count: Box::from(count),
            identifier: identifier_value,
            body: self.parse_proc_body(),
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
        Some(self.parse_proc_body())
    }

    fn parse_set(&self, symbol: &String) -> AST {
        AST::Assign {
            identifier: symbol.to_string(),
            value: Box::from(self.parse_atom()),
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
        let mut tokens = vec![];
        // args after given keyword
        if self.is_tok(&Token::Given) {
            // skip given
            self.next();
            self.skip(&Token::LParen);
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
        }
        return tokens;
    }

    fn parse_proc(&self, symbol: &String) -> AST {
        AST::Proc {
            identifier: symbol.to_string(),
            body: self.parse_proc_body(),
            args: self.parse_func_args(),
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
