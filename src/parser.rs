use crate::token::{Token, TokenWithContext};
use std::{iter::Peekable, slice::Iter};

pub fn parse(tokens: &[TokenWithContext]) {
    // let mut results = vec![];
    let mut peekable_tokens = tokens.iter().peekable();
    skip_initial_empty_lines(&mut peekable_tokens);
}

fn skip_initial_empty_lines(peekable_tokens: &mut Peekable<Iter<TokenWithContext>>) {
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
