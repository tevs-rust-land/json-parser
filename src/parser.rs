use crate::ast;
use crate::token::{Token, TokenWithContext};
use std::collections::HashMap;
use std::{iter::Peekable, slice::Iter};

fn process_token_to_ast(
    token: &TokenWithContext,
    peekable_tokens: &mut Peekable<Iter<TokenWithContext>>,
) -> ast::JSON {
    match &token.token {
        Token::DigitLiteral(_literal) => ast::JSON::NumberType,
        Token::StringLiteral(_literal) => ast::JSON::StringType,
        Token::True | Token::False => ast::JSON::Bool,
        Token::LeftBracket => {
            peekable_tokens.next();
            iterate::over_array(peekable_tokens)
        }
        Token::LeftBrace => {
            peekable_tokens.next();
            todo!()
        }

        _ => todo!(),
    }
}

pub fn parse(
    tokens: &[TokenWithContext],
) -> (std::vec::Vec<ast::JSON>, std::vec::Vec<ast::JSONError>) {
    let mut results: Vec<ast::JSON> = vec![];
    let mut errors: Vec<ast::JSONError> = vec![];
    let mut peekable_tokens = tokens.iter().peekable();
    skip_initial_new_lines(&mut peekable_tokens);

    while let Some(token) = peekable_tokens.peek() {
        let element = process_token_to_ast(token, &mut peekable_tokens);
        peekable_tokens.next();
        match element {
            ast::JSON::Error(err) => errors.push(err),
            element => results.push(element),
        }
    }

    (results, errors)
}

fn skip_initial_new_lines(peekable_tokens: &mut Peekable<Iter<TokenWithContext>>) {
    while let Some(token) = peekable_tokens.peek() {
        match token.token {
            Token::NextLine => {
                peekable_tokens.next();
                continue;
            }
            _ => {
                break;
            }
        }
    }
}

mod iterate {
    use super::*;
    use ast::{ArrayType, JSONError};
    pub fn over_array(peekable_tokens: &mut Peekable<Iter<TokenWithContext>>) -> ast::JSON {
        let mut array_body = vec![];
        while let Some(token) = peekable_tokens.peek() {
            match &token.token {
                Token::RightBracket => {
                    peekable_tokens.next();
                    return ast::JSON::Array(ArrayType { body: array_body });
                }
                _t => array_body.push(process_token_to_ast(token, peekable_tokens)),
            }
            peekable_tokens.next();
        }

        ast::JSON::Error(JSONError::UnterminatedArray)
    }

    pub fn over_object(peekable_tokens: &mut Peekable<Iter<TokenWithContext>>) -> ast::JSON {
        let mut object_body = vec![];
        while let Some(token) = peekable_tokens.next() {
            match &token.token {
                Token::RightBrace => {
                    peekable_tokens.next();
                    break;
                }
                _ => object_body.push(token),
            }
        }
        if object_body.len() % 2 != 0 {
            ast::JSON::Error(JSONError::UnterminatedObject)
        } else {
            todo!()
        }
    }

    fn map_array_of_tokens_to_object(tokens: &[TokenWithContext]) -> ast::JSON {
        let mut iter = tokens.rchunks(2);
        let mut object_body = HashMap::new();
        while let Some([key, value]) = iter.next() {
            let key = match &key.token {
                Token::StringLiteral(key) => key.to_owned(),
                _ => return ast::JSON::Error(JSONError::UnexprectedObjectKey),
            };
            object_body.insert(key, value);
        }
        ast::JSON::Object(ast::ObjectType { body: object_body })
    }
}

#[cfg(test)]
mod tests {
    use ast::JSONError;

    use super::*;
    use crate::ast;
    use crate::scanner;
    #[test]
    fn test_can_parse_number_type() {
        let source = r#"12"#;
        let (scanned_output, _errors) = scanner::scan(source);

        let (parsed_results, _errors) = parse(&scanned_output);
        assert_eq!(parsed_results, vec![ast::JSON::NumberType])
    }

    #[test]
    fn test_can_parse_string_type() {
        let source = r#""output""#;
        let (scanned_output, _errors) = scanner::scan(source);
        let (parsed_results, _errors) = parse(&scanned_output);
        assert_eq!(parsed_results, vec![ast::JSON::StringType])
    }
    #[test]
    fn test_can_parse_boolean_type() {
        let source = r#"true"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let (parsed_results, _errors) = parse(&scanned_output);
        assert_eq!(parsed_results, vec![ast::JSON::Bool]);

        let source = r#"false"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let (parsed_results, _errors) = parse(&scanned_output);
        assert_eq!(parsed_results, vec![ast::JSON::Bool])
    }

    #[test]
    fn test_can_parse_boolean_array_type() {
        let source = r#"[true, false]"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let (parsed_results, _errors) = parse(&scanned_output);
        let array_body = ast::ArrayType {
            body: vec![ast::JSON::Bool, ast::JSON::Bool],
        };
        let json_array = vec![ast::JSON::Array(array_body)];

        assert_eq!(parsed_results, json_array)
    }
    #[test]
    fn test_can_parse_string_array_type() {
        let source = r#"["tev", "codes"]"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let (parsed_results, _errors) = parse(&scanned_output);
        let array_body = ast::ArrayType {
            body: vec![ast::JSON::StringType, ast::JSON::StringType],
        };
        let json_array = vec![ast::JSON::Array(array_body)];
        assert_eq!(parsed_results, json_array)
    }

    #[test]
    fn test_can_parse_integer_array_type() {
        let source = r#"[20, 21]"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let (parsed_results, _errors) = parse(&scanned_output);
        let array_body = ast::ArrayType {
            body: vec![ast::JSON::NumberType, ast::JSON::NumberType],
        };
        let json_array = vec![ast::JSON::Array(array_body)];
        assert_eq!(parsed_results, json_array)
    }

    #[test]
    fn test_can_capture_unterminated_array() {
        let source = r#"[20, 21"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let (_parsed_results, errors) = parse(&scanned_output);
        assert_eq!(errors.len(), 1);
        let error = JSONError::UnterminatedArray;
        assert_eq!(errors, vec![error])
    }
    #[test]
    fn test_can_parse_object() {
        let source = r#"{"name": "12"}"#;
        let (scanned_output, _errors) = scanner::scan(source);
        println!("{:?}", scanned_output)
    }
}
