use crate::ast;
use crate::token::{Token, TokenWithContext};
use std::{iter::Peekable, slice::Iter};

pub fn parse(tokens: &[TokenWithContext]) -> std::vec::Vec<ast::JSON> {
    let mut results: Vec<ast::JSON> = vec![];
    let mut peekable_tokens = tokens.iter().peekable();
    skip_initial_new_lines(&mut peekable_tokens);

    while let Some(token) = peekable_tokens.peek() {
        let element = match &token.token {
            Token::DigitLiteral(_literal) => ast::JSON::NumberType,
            Token::StringLiteral(_literal) => ast::JSON::StringType,
            Token::True | Token::False => ast::JSON::Bool,
            Token::LeftBracket => {
                peekable_tokens.next();
                let json_array = iterate::over_array(&mut peekable_tokens);
                ast::JSON::Array(json_array)
            }
            _ => todo!(),
        };
        peekable_tokens.next();
        results.push(element);
    }

    results
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
    use ast::ArrayType;
    pub fn over_array(peekable_tokens: &mut Peekable<Iter<TokenWithContext>>) -> ast::ArrayType {
        let mut array_body = vec![];
        while let Some(token) = peekable_tokens.peek() {
            match &token.token {
                Token::RightBracket => {
                    peekable_tokens.next();
                    break;
                }
                token => {
                    peekable_tokens.next();
                    array_body.push(token.into())
                }
            }
        }
        ArrayType { body: array_body }
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
    use super::*;
    use crate::ast;
    use crate::scanner;
    #[test]
    fn test_can_parse_number_type() {
        let source = r#"12"#;
        let (scanned_output, _errors) = scanner::scan(source);

        let parsed_results = parse(&scanned_output);
        assert_eq!(parsed_results, vec![ast::JSON::NumberType])
    }

    #[test]
    fn test_can_parse_string_type() {
        let source = r#""output""#;
        let (scanned_output, _errors) = scanner::scan(source);
        let parsed_results = parse(&scanned_output);
        assert_eq!(parsed_results, vec![ast::JSON::StringType])
    }
    #[test]
    fn test_can_parse_boolean_type() {
        let source = r#"true"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let parsed_results = parse(&scanned_output);
        assert_eq!(parsed_results, vec![ast::JSON::Bool]);

        let source = r#"false"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let parsed_results = parse(&scanned_output);
        assert_eq!(parsed_results, vec![ast::JSON::Bool])
    }

    #[test]
    fn test_can_parse_boolean_array_type() {
        let source = r#"[true, false]"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let parsed_results = parse(&scanned_output);
        let inner_array = ast::ArrayType {
            body: vec![ast::JSON::Bool, ast::JSON::Bool],
        };
        let json_array = vec![ast::JSON::Array(inner_array)];

        assert_eq!(parsed_results, json_array)
    }
    #[test]
    fn test_can_parse_string_array_type() {
        let source = r#"["tev", "codes"]"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let parsed_results = parse(&scanned_output);
        let inner_array = ast::ArrayType {
            body: vec![ast::JSON::StringType, ast::JSON::StringType],
        };
        let json_array = vec![ast::JSON::Array(inner_array)];
        assert_eq!(parsed_results, json_array)
    }

    #[test]
    fn test_can_parse_integer_array_type() {
        let source = r#"[20, 21]"#;
        let (scanned_output, _errors) = scanner::scan(source);
        let parsed_results = parse(&scanned_output);
        let inner_array = ast::ArrayType {
            body: vec![ast::JSON::NumberType, ast::JSON::NumberType],
        };
        let json_array = vec![ast::JSON::Array(inner_array)];
        assert_eq!(parsed_results, json_array)
    }
}
