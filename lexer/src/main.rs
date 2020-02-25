extern crate regex;

mod file;
mod token_types;

use regex::Regex;
use token_types::TokenType;

fn main() {
    let root_dir = "/Users/jacobsansbury/prj/jsnns/eye".to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("No source file give.");
    }

    let first_source_path = args[1].to_string();
    let source_text = file::read_source_file(first_source_path, root_dir);

    println!("{:?}", make_tokens(source_text))
}

fn make_tokens(source_text: String) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = vec![];
    let mut pos: usize = 0;

    let num_regex = Regex::new(r"^[-]?\d+").unwrap();
    while pos <= source_text.len() - 1 {
        let test_str = &source_text[pos..source_text.len()];

        // println!("{}", test_str);

        if source_text.chars().nth(pos).unwrap() == ' ' {
            pos += 1;
        } else if Regex::new(r"^\+").unwrap().is_match(test_str) {
            pos += 1;
            tokens.push(TokenType::Add)
        } else if Regex::new(r"^proc").unwrap().is_match(test_str) {
            pos += 4;
            tokens.push(TokenType::Proc)
        } else if Regex::new(r"^;").unwrap().is_match(test_str) {
            pos += 1;
            tokens.push(TokenType::Semicolon);
        } else if num_regex.is_match(test_str) {
            let num = num_regex.find(test_str).unwrap().as_str();
            pos += num.len();
            tokens.push(TokenType::Number(num.parse().unwrap()))
        }
    }

    tokens
}
