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
    Struct(std::collections::HashMap<String, Types>),
    Function(std::collections::HashMap<String, Types>),
}
