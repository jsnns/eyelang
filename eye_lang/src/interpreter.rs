use crate::ast::BinaryOperator;
use crate::ast::PrimitiveValue;
use crate::ast::AST;

fn value_from_ast(ast: AST) -> PrimitiveValue {
    match ast {
        AST::Number { value } => PrimitiveValue::Num(value),
        _ => panic!("Operator value could not be determined {:?}", ast),
    }
}

fn apply_binary_operator(left: AST, right: AST, operator: BinaryOperator) -> PrimitiveValue {
    let left_value = value_from_ast(left);
    let right_value = value_from_ast(right);
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

fn run_ast(ast: Box<AST>) -> Option<PrimitiveValue> {
    match *ast {
        AST::Binary {
            operator,
            left,
            right,
        } => Some(apply_binary_operator(*left, *right, operator)),
        AST::Proc { symbol, value } => {
            for ast in value {
                run_ast(ast);
            }
            None
        }
        AST::Return { value } => {
            println!("Return value: {:?}", run_ast(value).unwrap());
            None
        }
        AST::Semicolon => None,
        AST::Bool { value } => Some(PrimitiveValue::Bool(value)),
        _ => panic!("AST branch not implemented {:?}", ast),
    }
}

pub fn interpret(root_program: AST) {
    match root_program {
        AST::Program { program } => {
            for ast in program {
                run_ast(ast);
            }
        }
        _ => panic!("root_program not of type AST::Program, {:?}", root_program),
    }
}
