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
            _ => todo!(),
        };
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
