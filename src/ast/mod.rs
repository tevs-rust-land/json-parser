use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum JSON {
    Bool,
    StringType,
    NumberType,
    Object(ObjectType),
    Array(ArrayType),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArrayType {
    pub body: Vec<JSON>,
}
pub struct StringType;
pub struct NumberType;
#[derive(Debug, PartialEq, Eq)]
pub struct ObjectType {
    pub body: HashMap<String, JSON>,
}
