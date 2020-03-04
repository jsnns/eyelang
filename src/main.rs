mod file;
mod interpreter;
mod lexer;
mod parser;
mod types;

use std::collections::HashMap;
use types::options::Options;
use types::symbol_store::SymbolStore;

fn main() {
    let root_dir = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let first_source_path = args[1].to_string();
        let source_text = file::read_source_file(first_source_path, root_dir);
        if let Ok(tokens) = lexer::tokenize(source_text) {
            let ast = parser::build_program(tokens);
            let symbols: SymbolStore = HashMap::new();
            interpreter::interpret(ast, symbols, &Options::default());
        }
    } else {
        println!("First argument must be a source file.")
    }
}
