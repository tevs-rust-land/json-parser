pub mod flow;

use crate::ast::{Array, Object, JSON};
pub trait Interpreter {
    fn run(self) -> String;
}

impl Interpreter for Array {
    fn run(self) -> String {
        "".to_string()
    }
}

impl Interpreter for Object {
    fn run(self) -> String {
        "".to_string()
    }
}

impl Interpreter for JSON {
    fn run(self) -> String {
        "".to_string()
    }
}
