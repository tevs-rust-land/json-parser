use std::collections::HashMap;

pub enum JSON {
    Bool,
    StringType,
    NumberType,
    Object(ObjectType),
    Array(ArrayType),
}

pub struct ArrayType {
    pub body: Vec<JSON>,
}
pub struct StringType;
pub struct NumberType;

pub struct ObjectType {
    body: HashMap<String, JSON>,
}
