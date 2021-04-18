use crate::ast;
use crate::token::{Token, TokenWithContext};
use std::{iter::Peekable, slice::Iter};

pub fn parse(
    tokens: &[TokenWithContext],
) -> (std::vec::Vec<ast::JSON>, std::vec::Vec<ast::JSONError>) {
    let mut results: Vec<ast::JSON> = vec![];
    let mut errors: Vec<ast::JSONError> = vec![];
    let mut peekable_tokens = tokens.iter().peekable();
    skip_initial_new_lines(&mut peekable_tokens);

    while let Some(token) = peekable_tokens.peek() {
        let element = match &token.token {
            Token::DigitLiteral(_literal) => ast::JSON::NumberType,
            Token::StringLiteral(_literal) => ast::JSON::StringType,
            Token::True | Token::False => ast::JSON::Bool,
            Token::LeftBracket => {
                peekable_tokens.next();
                iterate::over_array(&mut peekable_tokens)
            }
            _ => todo!(),
        };
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
        let mut was_array_closed = false;
        while let Some(token) = peekable_tokens.peek() {
            match &token.token {
                Token::RightBracket => {
                    was_array_closed = true;
                    peekable_tokens.next();
                    break;
                }
                token => {
                    peekable_tokens.next();
                    array_body.push(token.into())
                }
            }
        }

        match was_array_closed {
            true => ast::JSON::Array(ArrayType { body: array_body }),
            false => ast::JSON::Error(JSONError::UnterminatedArray),
        }
    }
}

impl From<&Token> for ast::JSON {
    fn from(token: &Token) -> Self {
        match token {
            Token::DigitLiteral(_literal) => ast::JSON::NumberType,
            Token::StringLiteral(_literal) => ast::JSON::StringType,
            Token::True | Token::False => ast::JSON::Bool,
            t => {
                println!("{:?}", t);
                todo!()
            }
        }
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
}
