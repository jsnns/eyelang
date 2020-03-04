use crate::types::primitive_value::PrimitiveValue;

#[derive(Clone)]
pub struct Options {
    pub print_fn: fn(a: PrimitiveValue),
    pub debug: bool,
}

impl Options {
    #[allow(dead_code)]
    pub fn debug() -> Options {
        Options {
            print_fn: debug_print,
            debug: true,
        }
    }

    pub fn default() -> Options {
        Options {
            print_fn: default_print,
            debug: false,
        }
    }
}

fn debug_print(a: PrimitiveValue) {
    default_print(a);
}

fn default_print(a: PrimitiveValue) {
    println!("{}", a.to_string());
}
