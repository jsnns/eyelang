use crate::types::ast::AST;
use crate::types::binary_operator::BinaryOperator;
use crate::types::body::Block;
use crate::types::error::RuntimeError;
use crate::types::primitive_type::PrimitiveValue;

use std::collections::HashMap;
use std::time::Instant;

/**
 * Recursively run an AST program
 */
pub fn interpret(root_program: AST, mut symbols: HashMap<String, PrimitiveValue>) {
    if let AST::Program { program } = root_program {
        let now = Instant::now();
        match run_body_and_return(program, &mut symbols) {
            Err(error) => println!("Runtime Error! {}", error.message),
            Ok(..) => println!("Done in {}ms", now.elapsed().as_millis()),
        };
    } else {
        panic!("root_program not of type AST::Program, {:?}", root_program);
    }
}

// Get a primitive value from an AST
fn value_from_ast(
    ast: AST,
    symbols: &mut HashMap<String, PrimitiveValue>,
) -> Result<PrimitiveValue, RuntimeError> {
    let new_ast = ast.clone();
    match ast {
        AST::Number { value } => Ok(PrimitiveValue::Num(value)),
        AST::Bool { value } => Ok(PrimitiveValue::Bool(value)),
        AST::Str { value } => Ok(PrimitiveValue::Str(value)),
        AST::Binary {
            left,
            right,
            operator,
        } => apply_binary_operator(*left, *right, operator, symbols),
        AST::Call { func, args: _ } => {
            if let Some(value) = run_ast(new_ast, symbols)? {
                return Ok(value);
            } else {
                Err(RuntimeError {
                    message: format!("Function {} didn't return value", func),
                })
            }
        }
        AST::Symbol { name } => {
            if let Some(value) = symbols.get(&name) {
                let new_value = value.clone();
                return Ok(new_value);
            } else {
                Err(RuntimeError {
                    message: format!("Could not get value from Symbol: {:?}", name),
                })
            }
        }
        _ => Err(RuntimeError {
            message: format!("Value of AST could not be determined {:?}", ast),
        }),
    }
}

/**
 * Get value from asts and then apply binary operator
 */
fn apply_binary_operator(
    left: AST,
    right: AST,
    operator: BinaryOperator,
    symbols: &mut HashMap<String, PrimitiveValue>,
) -> Result<PrimitiveValue, RuntimeError> {
    let left_value = value_from_ast(left, symbols)?;
    let right_value = value_from_ast(right, symbols)?;
    match operator {
        BinaryOperator::Add => Ok(left_value.add(right_value)?),
        BinaryOperator::Subtract => Ok(left_value.subtract(right_value)?),
        BinaryOperator::Multiply => Ok(left_value.multiply(right_value)?),
        BinaryOperator::IsEq => Ok(left_value.is_equal(right_value)?),
        _ => panic!("Operator not implemented {}", operator.to_string()),
    }
}

/**
 * Run given set of ASTs and return any value returned by the ASTs
 */
fn run_body_and_return(
    body: Vec<Box<AST>>,
    symbols: &mut HashMap<String, PrimitiveValue>,
) -> Result<Option<PrimitiveValue>, RuntimeError> {
    for ast in body {
        let new_ast = *ast.clone();
        match *ast {
            AST::Return { value: _ } => return run_ast(new_ast, symbols),
            AST::If {
                this: _,
                elifs: _,
                el: _,
            } => {
                if let Some(val) = run_ast(new_ast, symbols)? {
                    return Ok(Some(val));
                }
            }
            _ => {
                run_ast(new_ast, symbols)?;
            }
        }
    }

    Ok(None)
}

fn run_ast(
    ast: AST,
    symbols: &mut HashMap<String, PrimitiveValue>,
) -> Result<Option<PrimitiveValue>, RuntimeError> {
    match ast {
        AST::Number { value } => Ok(Some(PrimitiveValue::Num(value))),
        AST::Bool { value } => Ok(Some(PrimitiveValue::Bool(value))),
        AST::Str { value } => Ok(Some(PrimitiveValue::Str(value))),
        AST::Binary {
            operator,
            left,
            right,
        } => Ok(Some(apply_binary_operator(
            *left, *right, operator, symbols,
        )?)),
        AST::Proc {
            symbol,
            value,
            args,
        } => {
            let mut body: Vec<Box<AST>> = vec![];

            for ast in value {
                body.push(Box::from(*ast));
            }

            symbols.insert(
                symbol,
                PrimitiveValue::Block(Block {
                    body: body,
                    args: args,
                }),
            );
            Ok(None)
        }
        AST::Call { func, args } => {
            if !symbols.contains_key(&func) {
                return Err(RuntimeError {
                    message: format!("Symbol {} does not exist.", func),
                });
            }

            if let Some(PrimitiveValue::Block(block)) = symbols.get(&func) {
                // this sets up the function's "scope"
                let mut f_symbols = symbols.clone();

                let args_requested = block.args.clone();
                let args_given = args;

                assert_eq!(args_given.len(), args_requested.len());

                for i in 0..args_requested.len() {
                    if let Some(value) = run_ast(*args_given[i].clone(), &mut symbols.clone())? {
                        f_symbols.insert(args_requested[i].clone(), value);
                    } else {
                        return Err(RuntimeError {
                            message: format!("Could not evaluate arguemnt {:?}", args_given[i]),
                        });
                    }
                }

                return run_body_and_return(block.body.clone(), &mut f_symbols);
            }

            Ok(None)
        }
        AST::Return { value } => run_ast(*value, symbols),
        AST::Semicolon => Ok(None),
        AST::Assign { symbol, value } => {
            if let Some(symbol_value) = run_ast(*value, symbols)? {
                symbols.insert(symbol, symbol_value);
            }

            Ok(None)
        }
        AST::Print { value } => {
            if let Some(value) = run_ast(*value, symbols)? {
                println!("{:?}", value);
            } else {
                println!("''");
            }
            Ok(None)
        }
        AST::Symbol { name } => {
            if let Some(value) = symbols.get(&name) {
                let new_value = value.clone();
                return Ok(Some(new_value));
            }

            Ok(None)
        }
        AST::If { this, elifs, el } => {
            if let Some(PrimitiveValue::Bool(val)) =
                run_ast(*this.conditional, &mut symbols.clone())?
            {
                if val {
                    // run if statement body and return if needed
                    return run_body_and_return(this.body, &mut symbols.clone());
                }

                // go through each elif
                if let Some(elifs) = elifs {
                    for elif in elifs {
                        if let Some(PrimitiveValue::Bool(elif_val)) =
                            run_ast(*elif.conditional, &mut symbols.clone())?
                        {
                            if elif_val {
                                return run_body_and_return(
                                    elif.body.clone(),
                                    &mut symbols.clone(),
                                );
                            }
                        }
                    }
                }

                // if we fall through to here
                // we haven't found anything
                if let Some(el) = el {
                    return run_body_and_return(el.clone(), &mut symbols.clone());
                }
            }
            Ok(None)
        }
        AST::EOF => Ok(None),
        AST::Program { program: _ } => Err(RuntimeError {
            message: format!("Found program in AST."),
        }),
    }
}
