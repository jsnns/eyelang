use crate::types::ast::AST;
use crate::types::binary_operator::BinaryOperator;
use crate::types::body::Block;
use crate::types::primitive_type::PrimitiveValue;

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
        AST::Symbol { name } => {
            if let Some(value) = symbols.get(&name) {
                let new_value = value.clone();
                return new_value;
            } else {
                panic!("Could not get value from Symbol: {:?}", name);
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
        AST::Proc {
            symbol,
            value,
            args,
        } => {
            let mut body: Vec<AST> = vec![];

            for ast in value {
                body.push(*ast);
            }

            symbols.insert(
                symbol,
                PrimitiveValue::Block(Block {
                    body: body,
                    args: args,
                }),
            );
            None
        }
        AST::Call { func, args } => {
            if !symbols.contains_key(&func) {
                panic!("Symbol {} does not exist.", func);
            }

            if let Some(PrimitiveValue::Block(block)) = symbols.get(&func) {
                // this sets up the function's "scope"
                let mut f_symbols = symbols.clone();

                let args_requested = block.args.clone();
                let args_given = args;

                assert_eq!(args_given.len(), args_requested.len());

                for i in 0..args_requested.len() {
                    if let Some(value) = run_ast(*args_given[i].clone(), &mut symbols.clone()) {
                        f_symbols.insert(args_requested[i].clone(), value);
                    } else {
                        panic!("Could not evalue arguemnt {:?}", args_given[i]);
                    }
                }

                for ast in block.body.clone() {
                    let new_ast = ast.clone();
                    match ast {
                        AST::Return { value: _ } => return run_ast(new_ast, &mut f_symbols),
                        _ => {
                            run_ast(new_ast, &mut f_symbols);
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
        AST::Assign { symbol, value } => {
            if let Some(symbol_value) = run_ast(*value, symbols) {
                symbols.insert(symbol, symbol_value);
            }

            None
        }
        AST::Print { value } => {
            if let Some(value) = run_ast(*value, symbols) {
                println!("{:?}", value);
            } else {
                println!("Printed nothing");
            }
            None
        }
        AST::Symbol { name } => {
            if let Some(value) = symbols.get(&name) {
                let new_value = value.clone();
                return Some(new_value);
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
