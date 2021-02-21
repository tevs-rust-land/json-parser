use crate::ast::JSON;
use crate::interpreter::Interpreter;
struct Flow(JSON);

impl Interpreter for Flow {
    fn execute(self) -> String {
        let element = self.0;
        match element {
            JSON::NumberType => "number".to_string(),
            JSON::StringType => "string".to_string(),
            _ => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_json() {
        let json_value = JSON::NumberType;
        let flow_interpreter = Flow(json_value);
        let result = flow_interpreter.execute();
        assert_eq!(format!("number"), result)
    }
    #[test]
    fn test_string_json() {
        let json_value = JSON::StringType;
        let flow_interpreter = Flow(json_value);
        let result = flow_interpreter.execute();
        assert_eq!(format!("string"), result)
    }
}
