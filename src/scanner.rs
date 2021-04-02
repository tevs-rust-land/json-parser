use crate::token::{self, Position, Token, TokenWithContext};
use std::iter::Peekable;
use std::str;

#[derive(Debug, PartialEq)]
pub enum ScannerError {
    MissingStringTerminator(Position),
}

struct Scanner<'a> {
    current_position: Position,
    current_lexeme: String,
    source: Peekable<str::Chars<'a>>,
}

impl<'a> Scanner<'a> {
    fn initialize(source: &'a str) -> Scanner {
        Scanner {
            current_lexeme: "".into(),
            current_position: Position::initial(),
            source: source.chars().into_iter().peekable(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.source.next();
        if let Some(c) = next {
            self.current_lexeme.push(c);
            if c == '\n' {
                self.current_position.increment_line();
            } else {
                self.current_position.increment_column();
            }
        }
        next
    }

    fn add_context(&mut self, token: Token, initial_position: Position) -> TokenWithContext {
        TokenWithContext {
            token,
            lexeme: self.current_lexeme.clone(),
            position: initial_position,
        }
    }

    fn scan_next(&mut self) -> Option<Result<TokenWithContext, ScannerError>> {
        let initial_position = self.current_position;
        self.current_lexeme.clear();
        let next_char = match self.advance() {
            Some(c) => c,
            None => return None,
        };

        let result = match next_char {
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            c if token::is_nextline(c) => Ok(Token::NextLine),
            c if token::is_whitespace(c) => Ok(Token::Whitespace),
            c if token::is_digit(c) => Ok(self.digit()),
            '"' => self.string(),
            _ => unimplemented!(),
        };

        Some(result.map(|token| self.add_context(token, initial_position)))
    }
    fn peek_check(&mut self, check: &dyn Fn(char) -> bool) -> bool {
        match self.source.peek() {
            Some(&c) => check(c),
            None => false,
        }
    }
    fn advance_if_match(&mut self, expected: char) -> bool {
        if self.peek_check(&|c| c == expected) {
            let _ = self.advance();
            true
        } else {
            false
        }
    }
    fn advance_while(&mut self, condition: &dyn Fn(char) -> bool) {
        while self.peek_check(condition) {
            self.advance();
        }
    }
    fn string(&mut self) -> Result<Token, ScannerError> {
        self.advance_while(&|c| c != '"' && c != '\n');
        if !self.advance_if_match('"') {
            return Err(ScannerError::MissingStringTerminator(self.current_position));
        }
        let literal_length = self.current_lexeme.len() - 2;
        let literal: String = self
            .current_lexeme
            .chars()
            .skip(1)
            .take(literal_length)
            .collect();

        Ok(Token::StringLiteral(literal))
    }

    fn digit(&mut self) -> Token {
        self.advance_while(&|c| c != '\n' && c != ',');
        let literal_length = self.current_lexeme.len();
        let num = self.current_lexeme.chars().take(literal_length).collect();
        Token::DigitLiteral(num)
    }
}

struct TokensIterator<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Iterator for TokensIterator<'a> {
    type Item = Result<TokenWithContext, ScannerError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scanner.scan_next()
    }
}

pub fn scan_into_iterator<'a>(
    source: &'a str,
) -> impl Iterator<Item = Result<TokenWithContext, ScannerError>> + 'a {
    TokensIterator {
        scanner: Scanner::initialize(source),
    }
}

pub fn scan(source: &str) -> (Vec<TokenWithContext>, Vec<ScannerError>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for result in scan_into_iterator(source) {
        match result {
            Ok(token_with_context) => match token_with_context.token {
                Token::Whitespace => {}
                _ => tokens.push(token_with_context),
            },
            Err(error) => errors.push(error),
        }
    }
    (tokens, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scanner_returns_missing_string_terminator_error_for_unterminated_string_value() {
        let json_string = r#"
        {
            "name": "Tev 
        }
        "#;
        let (_tokens, scanner_errors) = scan(&json_string);

        assert_eq!(scanner_errors.len(), 1);
        assert_eq!(
            vec![ScannerError::MissingStringTerminator(Position {
                column: 26,
                line: 3
            })],
            scanner_errors
        )
    }
    #[test]

    fn test_should_successfully_scan_and_produce_tokens_for_valid_json_without_error() {
        let json_string = r#"
        {
            "name": "Tev"
        }
        "#;
        let (_tokens, scanner_errors) = scan(&json_string);

        assert_eq!(scanner_errors.len(), 0);
    }
}
