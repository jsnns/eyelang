use crate::types::ast::FunctionBody;
use crate::types::ast::AST;
use crate::types::binary_operator::BinaryOperator;
use crate::types::error::RuntimeError;
use crate::types::primitive_value::PrimitiveValue;
use crate::types::symbol_store::SymbolStore;

use std::time::Instant;

/**
 * Recursively run an AST program
 */
pub fn interpret(root_program: AST, mut symbols: SymbolStore) {
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
fn value_from_ast(ast: AST, symbols: &mut SymbolStore) -> Result<PrimitiveValue, RuntimeError> {
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
        AST::Call {
            identifier,
            args: _,
        } => {
            if let Some(value) = run_ast(new_ast, symbols)? {
                return Ok(value);
            } else {
                Err(RuntimeError {
                    message: format!("Function {} didn't return value", identifier),
                })
            }
        }
        AST::Symbol { identifier } => {
            if let Some(value) = symbols.get(&identifier) {
                let new_value = value.clone();
                return Ok(new_value);
            } else {
                Err(RuntimeError {
                    message: format!("Could not get value from Symbol: {:?}", identifier),
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
    symbols: &mut SymbolStore,
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
    symbols: &mut SymbolStore,
) -> Result<Option<PrimitiveValue>, RuntimeError> {
    for ast in body {
        let new_ast = *ast.clone();
        match *ast {
            AST::Return { value: _ } => return run_ast(new_ast, symbols),
            AST::Do {
                count: _,
                body: _,
                identifier: _,
            } => {
                if let Some(val) = run_ast(new_ast, symbols)? {
                    return Ok(Some(val));
                }
            }
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

fn run_ast(ast: AST, symbols: &mut SymbolStore) -> Result<Option<PrimitiveValue>, RuntimeError> {
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
            identifier,
            body,
            args,
        } => {
            symbols.insert(
                identifier,
                PrimitiveValue::Function(FunctionBody {
                    body: body,
                    args: args,
                }),
            );
            Ok(None)
        }
        AST::Call { identifier, args } => {
            if !symbols.contains_key(&identifier) {
                return Err(RuntimeError {
                    message: format!("Symbol {} does not exist.", identifier),
                });
            }

            if let Some(PrimitiveValue::Function(block)) = symbols.get(&identifier) {
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
        AST::Assign { identifier, value } => {
            if let Some(symbol_value) = run_ast(*value, symbols)? {
                symbols.insert(identifier, symbol_value);
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
        AST::Symbol { identifier } => {
            if let Some(value) = symbols.get(&identifier) {
                let new_value = value.clone();
                return Ok(Some(new_value));
            } else {
                Err(RuntimeError {
                    message: format!("Tried to access undefined symbol: {}", identifier),
                })
            }
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
        AST::Do {
            count,
            body,
            identifier,
        } => {
            if let Ok(PrimitiveValue::Num(count)) = value_from_ast(*count, &mut symbols.clone()) {
                let mut f_symbols = symbols.clone();
                for i in 0..count {
                    f_symbols.insert(identifier.clone(), PrimitiveValue::Num(i));
                    // only return if a return value is given
                    if let Some(return_value) = run_body_and_return(body.clone(), &mut f_symbols)? {
                        return Ok(Some(return_value));
                    }
                }
            }
            return Ok(None);
        }
        AST::EOF => Ok(None),
        AST::Program { program: _ } => Err(RuntimeError {
            message: format!("Found program in AST."),
        }),
    }
}
