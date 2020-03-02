use crate::ast::BinaryOperator;
use crate::ast::PrimitiveValue;
use crate::ast::AST;

use std::collections::HashMap;

fn value_from_ast(ast: AST, symbols: &mut HashMap<String, PrimitiveValue>) -> PrimitiveValue {
    let new_ast = ast.clone();
    match ast {
        AST::Number { value } => PrimitiveValue::Num(value),
        AST::Call { func, args: _ } => {
            if let Some(value) = run_ast(new_ast, symbols) {
                return value;
            } else {
                panic!("Function {} didn't return value", func);
            }
        }
        _ => panic!("Operator value could not be determined {:?}", ast),
    }
}

fn apply_binary_operator(
    left: AST,
    right: AST,
    operator: BinaryOperator,
    symbols: &mut HashMap<String, PrimitiveValue>,
) -> PrimitiveValue {
    let left_value = value_from_ast(left, symbols);
    let right_value = value_from_ast(right, symbols);
    match operator {
        BinaryOperator::Add => PrimitiveValue::get_operator_value(left_value.add(right_value)),
        BinaryOperator::Subtract => {
            PrimitiveValue::get_operator_value(left_value.subtract(right_value))
        }
        BinaryOperator::Multiply => {
            PrimitiveValue::get_operator_value(left_value.multiply(right_value))
        }
        _ => panic!("Operator not implemented {}", operator.to_string()),
    }
}

fn run_ast(ast: AST, symbols: &mut HashMap<String, PrimitiveValue>) -> Option<PrimitiveValue> {
    match ast {
        AST::Binary {
            operator,
            left,
            right,
        } => Some(apply_binary_operator(*left, *right, operator, symbols)),
        AST::Proc { symbol, value } => {
            let mut body: Vec<AST> = vec![];

            for ast in value {
                body.push(*ast);
            }

            symbols.insert(symbol, PrimitiveValue::Block(body));
            None
        }
        AST::Call { func, args } => {
            //TODO: implement args
            assert!(args.len() == 0);

            if !symbols.contains_key(&func) {
                panic!("Symbol {} does not exist.", func);
            }

            if let Some(PrimitiveValue::Block(f_body)) = symbols.get(&func) {
                for ast in f_body {
                    let new_ast = ast.clone();
                    match ast {
                        AST::Return { value: _ } => return run_ast(new_ast, &mut symbols.clone()),
                        _ => {
                            run_ast(new_ast, &mut symbols.clone());
                        }
                    }
                }
            }

            None
        }
        AST::Return { value } => run_ast(*value, symbols),
        AST::Semicolon => None,
        AST::Bool { value } => Some(PrimitiveValue::Bool(value)),
        AST::Number { value } => Some(PrimitiveValue::Num(value)),
        AST::Print { value } => {
            if let Some(value) = run_ast(*value, symbols) {
                println!("{:?}", value);
            }
            None
        }
        _ => panic!("AST branch not implemented {:?}", ast),
    }
}

pub fn interpret(root_program: AST, mut symbols: HashMap<String, PrimitiveValue>) {
    match root_program {
        AST::Program { program } => {
            for ast in program {
                run_ast(*ast, &mut symbols);
            }
        }
        _ => panic!("root_program not of type AST::Program, {:?}", root_program),
    }
}
