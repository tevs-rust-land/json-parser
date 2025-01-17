#[derive(Debug, PartialEq, Eq)]
pub enum JSON {
    Bool,
    StringType(String),
    NumberType,
    Object(ObjectType),
    Array(ArrayType),
    Error(JSONError),
    Colon,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArrayType {
    pub body: Vec<JSON>,
}
pub struct StringType;
pub struct NumberType;
#[derive(Debug, PartialEq, Eq)]
pub struct ObjectType {
    pub body: Vec<JSON>,
}

#[derive(Debug, PartialEq, Eq)]

pub enum JSONError {
    UnterminatedArray,
    UnterminatedObject,
}
