pub mod flow;

pub trait Interpreter {
    fn execute(self) -> String;
}
