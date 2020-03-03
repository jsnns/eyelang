use crate::types::primitive_type::PrimitiveValue;
use std::collections::HashMap;

pub type Identifier = String;
pub type SymbolStore = HashMap<Identifier, PrimitiveValue>;
