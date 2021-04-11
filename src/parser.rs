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
}
