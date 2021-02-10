use std::collections::HashMap;

pub enum JSON {
    String,
    Number,
    Object(Object),
    Array(Array),
}

pub struct Object {
    pub body: HashMap<String, JSON>,
}

pub struct Array {
    pub body: Vec<JSON>,
}
