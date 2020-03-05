#[cfg(test)]
mod test {
    use crate::interpreter::interpret;
    use crate::lexer::tokenize;
    use crate::parser::build_program;
    use crate::types::{ast, options, primitive_value, symbol_store};
    #[test]
    fn print_neg_number() {
        let program = setup_program("print -1;");
        let mut options = options::Options::debug();
        let symbols = symbol_store::SymbolStore::new();

        fn print_fn(a: primitive_value::PrimitiveValue) {
            // check the 10th fib number
            check_print(primitive_value::PrimitiveValue::Num(-1))(a)
        }
        options.print_fn = print_fn;

        interpret(program, symbols, &options);
    }

    #[test]
    fn tenth_fb() {
        let program = setup_program(
            "define fib to be {
            if n is 0 {
                return 0;
            } else if n is 1 {
                return 1;
            } else {
                return fib(n-1) + fib(n-2);
            }
        } given (n);
        
        print run fib given (10);",
        );

        let mut options = options::Options::debug();

        fn print_fn(a: primitive_value::PrimitiveValue) {
            // check the 10th fib number
            check_print(primitive_value::PrimitiveValue::Num(55))(a)
        }
        options.print_fn = print_fn;
        interpret(program, symbol_store::create_symbol_store(), &options);
    }

    fn setup_program(s: &str) -> ast::AST {
        let tokens = tokenize(s.to_string()).unwrap();
        build_program(tokens)
    }

    fn check_print(
        b: primitive_value::PrimitiveValue,
    ) -> Box<dyn Fn(primitive_value::PrimitiveValue)> {
        Box::new(move |a: primitive_value::PrimitiveValue| {
            (options::Options::debug().print_fn)(a.clone());
            assert_eq!(a, b);
        })
    }
}
