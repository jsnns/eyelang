mod ast;
mod error;
mod file;
mod interpreter;
mod lexer;
mod parser;
mod token;

use ast::PrimitiveValue;
use std::collections::HashMap;

fn main() {
    let root_dir = "/Users/jacobsansbury/prj/jsnns/eye".to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("No source file give.");
    }

    let first_source_path = args[1].to_string();
    let source_text = file::read_source_file(first_source_path, root_dir);
    if let Ok(tokens) = lexer::tokenize(source_text) {
        let ast = parser::parse_tokens(tokens);

        let symbols: HashMap<String, PrimitiveValue> = HashMap::new();
        // println!("\nStart program === ===");
        interpreter::interpret(ast, symbols);
    }
}
