use crate::ast::{ArrayType, JSON};
use crate::interpreter::Interpreter;
struct Flow<'a>(&'a JSON);

impl Interpreter for Flow<'_> {
    fn execute(self) -> String {
        let element = self.0;
        match element {
            JSON::Bool => "boolean".to_string(),
            JSON::NumberType => "number".to_string(),
            JSON::StringType => "string".to_string(),
            JSON::Array(arr) => iterate_array(arr),
            _ => "".to_string(),
        }
    }
}

fn iterate_array(arr: &ArrayType) -> String {
    let mut result = "Array<".to_string();
    if let Some((last, elements)) = arr.body.split_last() {
        let loop_result: String = elements
            .iter()
            .map(|x| {
                let flow_element = Flow(x);
                format!("{} | ", flow_element.execute())
            })
            .collect();
        result = format!("{}{}", result, loop_result);
        let last_element = Flow(last);
        result.push_str(&last_element.execute());
    }
    result.push_str(">");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_json() {
        let json_value = JSON::NumberType;
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!(format!("number"), result)
    }
    #[test]
    fn test_string_json() {
        let json_value = JSON::StringType;
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!(format!("string"), result)
    }

    #[test]
    fn test_array_json() {
        let body = vec![JSON::StringType, JSON::NumberType];
        let array = ArrayType { body };
        let json_value = JSON::Array(array);
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!(format!("Array<string | number>"), result)
    }

    #[test]
    fn test_single_array_element() {
        let body = vec![JSON::StringType];
        let array = ArrayType { body };
        let json_value = JSON::Array(array);
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!(format!("Array<string>"), result)
    }

    #[test]
    fn test_empty_array() {
        let body = vec![];
        let array = ArrayType { body };
        let json_value = JSON::Array(array);
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!(format!("Array<>"), result)
    }
}
