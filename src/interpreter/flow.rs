use crate::ast::{ArrayType, ObjectType, JSON};
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
            JSON::Object(object) => iterate_object(object),
        }
    }
}

fn iterate_object(object: &ObjectType) -> String {
    let mut result = "{{ ".to_string();
    for (key, value) in &object.body {
        result.push_str(key);
        result.push_str(" : ");
        let flow_element = Flow(value);
        result.push_str(&flow_element.execute());
        result.push_str("; ");
    }
    result.push_str("}}");
    result
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
    result.push('>');
    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_int_json() {
        let json_value = JSON::NumberType;
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!("number".to_string(), result)
    }
    #[test]
    fn test_string_json() {
        let json_value = JSON::StringType;
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!("string".to_string(), result)
    }

    #[test]
    fn test_array_json() {
        let body = vec![JSON::StringType, JSON::NumberType];
        let array = ArrayType { body };
        let json_value = JSON::Array(array);
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!("Array<string | number>".to_string(), result)
    }

    #[test]
    fn test_single_array_element() {
        let body = vec![JSON::StringType];
        let array = ArrayType { body };
        let json_value = JSON::Array(array);
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!("Array<string>".to_string(), result)
    }

    #[test]
    fn test_empty_array() {
        let body = vec![];
        let array = ArrayType { body };
        let json_value = JSON::Array(array);
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!("Array<>".to_string(), result)
    }

    #[test]
    fn test_object_with_single_property() {
        let mut body: HashMap<String, JSON> = HashMap::new();
        body.insert("passed".to_string(), JSON::Bool);
        let object = ObjectType { body };
        let json_value = JSON::Object(object);
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();
        assert_eq!("{{ passed : boolean; }}".to_string(), result)
    }

    #[test]
    fn test_object_with_multiple_properties() {
        let mut body: HashMap<String, JSON> = HashMap::new();
        body.insert("passed".to_string(), JSON::Bool);
        body.insert("age".to_string(), JSON::NumberType);
        let array_body = vec![JSON::StringType];
        let array = ArrayType { body: array_body };
        let photos_array = JSON::Array(array);
        body.insert("photos".to_string(), photos_array);
        let object = ObjectType { body };
        let json_value = JSON::Object(object);
        let flow_interpreter = Flow(&json_value);
        let result = flow_interpreter.execute();

        assert!(result.contains("passed : boolean;"));
        assert!(result.contains("photos : Array<string>;"));
        assert!(result.contains("age : number;"));
    }
}
