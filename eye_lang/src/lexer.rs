extern crate regex;
use regex::Regex;

use crate::ast::BinaryOperator;
use crate::error::TokenError;
use crate::token::Token;

struct Position {
    index: u64,
    text: String,
}

impl Position {
    fn increment_by_str(&mut self, s: String) {
        self.index += s.len() as u64;
    }

    fn increment(&mut self, n: u64) {
        self.index += n;
    }

    fn next(&self) -> Box<String> {
        Box::new(self.text[(self.index as usize)..self.text.len()].to_string())
    }

    fn has_chars_left(&self) -> bool {
        self.text.len() > (self.index as usize)
    }

    fn current_char_is(&self, c: char) -> bool {
        self.current_char() == c
    }

    fn is_whitespace(&self) -> bool {
        self.current_char_is(' ') || self.current_char_is('\n') || self.current_char_is('\t')
    }

    fn current_char(&self) -> char {
        self.text
            .chars()
            .nth(self.index as usize)
            .unwrap_or_default()
    }

    fn re_find(&self, re: &Result<regex::Regex, regex::Error>) -> Result<String, TokenError> {
        if let Ok(regex) = re {
            let data = self.next();
            if let Some(result) = regex.find(&data) {
                Ok(result.as_str().to_string())
            } else {
                Err(TokenError)
            }
        } else {
            Ok("".to_string())
        }
    }
}

fn is_match(next_data_str: &Box<String>, re: &Result<regex::Regex, regex::Error>) -> bool {
    if let Ok(regex) = re {
        regex.is_match(&next_data_str)
    } else {
        false
    }
}

pub fn tokenize(source_text: String) -> Result<Vec<Token>, TokenError> {
    let mut tokens: Vec<Token> = vec![];
    let mut data = Position {
        index: 0,
        text: source_text,
    };

    let num_regex_result = Regex::new(r"^[-]?\d+");
    let symbol_regex_result = Regex::new(r"^[A-z][A-z0-9_]*");
    let type_regex_result = Regex::new(r"^: [0-9A-z]+");
    // TODO: this is gross
    while data.has_chars_left() {
        let next_data_str = data.next();

        if let Some(token) = match data.current_char() {
            '+' => Some(Token::Operator(BinaryOperator::Add)),
            '-' => Some(Token::Operator(BinaryOperator::Subtract)),
            '*' => Some(Token::Operator(BinaryOperator::Multiply)),
            '/' => Some(Token::Operator(BinaryOperator::Divide)),
            ';' => Some(Token::Semicolon),
            '}' => Some(Token::RBrace),
            '{' => Some(Token::LBrace),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ',' => Some(Token::Comma),
            _ => None,
        } {
            data.increment(1);
            tokens.push(token)
        }
        // match whitespace characters
        else if data.is_whitespace() {
            data.increment(1);
        }
        // match keywords
        else if is_match(&next_data_str, &Regex::new(r"^proc")) {
            data.increment(4);
            tokens.push(Token::Proc);
        } else if is_match(&next_data_str, &Regex::new(r"^main")) {
            data.increment(4);
            tokens.push(Token::Main);
        } else if is_match(&next_data_str, &Regex::new(r"^return")) {
            data.increment(6);
            tokens.push(Token::Return);
        } else if is_match(&next_data_str, &Regex::new(r"^true")) {
            data.increment(4);
            tokens.push(Token::Bool(true));
        } else if is_match(&next_data_str, &Regex::new(r"^false")) {
            data.increment(5);
            tokens.push(Token::Bool(false));
        } else if is_match(&next_data_str, &Regex::new(r"^print")) {
            data.increment(5);
            tokens.push(Token::Print);
        }
        // variable sequences ie numbers, symbols, strings
        else if is_match(&next_data_str, &num_regex_result) {
            let num = data
                .re_find(&num_regex_result)
                .unwrap_or("".to_string())
                .to_string();
            data.increment_by_str(num.clone());
            tokens.push(Token::Number(num.parse().unwrap()));
        } else if is_match(&next_data_str, &symbol_regex_result) {
            if let Ok(symbol_name) = data.re_find(&symbol_regex_result) {
                data.increment_by_str(symbol_name.clone());
                tokens.push(Token::Symbol(symbol_name));
            }
        } else if is_match(&next_data_str, &type_regex_result) {
            let type_value = data
                .re_find(&type_regex_result)
                .unwrap_or("".to_string())
                .to_string();
            data.increment_by_str(type_value.clone());

            let value_without_colon = type_value[1..type_value.len()].to_string();
            tokens.push(Token::Type(value_without_colon));
        } else {
            panic!("Could not find token for {}", next_data_str);
        }
    }

    Ok(tokens)
}
