use crate::types::primitive_value::PrimitiveValue;
use std::collections::HashMap;

pub type Identifier = String;
pub type SymbolStore = HashMap<Identifier, PrimitiveValue>;

pub fn create_symbol_store() -> SymbolStore {
    HashMap::new()
}
