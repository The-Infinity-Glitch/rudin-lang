#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Void,
    Int,
    Float,
    String,
    Bool,
    Char,
    Array(Vec<Types>),
    Tuple(Vec<Types>),
    Vector(Vec<Types>),
    Struct,
    Class,
    Function(std::collections::HashMap<String, Types>),
    Custom(String),
    Unknown,
}
