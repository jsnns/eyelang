mod error;
mod file;
mod lexer;
mod token;

fn main() {
    let root_dir = "/Users/jacobsansbury/prj/jsnns/eye".to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        panic!("No source file give.");
    }

    let first_source_path = args[1].to_string();
    let source_text = file::read_source_file(first_source_path, root_dir);

    println!("{:?}", lexer::tokenize(source_text));
}
