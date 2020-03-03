use crate::types::ast::AST;

#[derive(Debug, Clone)]
pub struct Block {
    pub body: Vec<Box<AST>>,
    pub args: Vec<String>,
}
