use itertools::{multipeek, MultiPeek};
use std::str;

pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    SemiColon,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    fn initial() -> Position {
        Position { line: 1, column: 1 }
    }

    fn increment_column(&mut self) {
        self.column += 1;
    }

    fn increment_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

pub struct TokenWithContext {
    pub token: Token,
    pub lexeme: String,
    pub position: Position,
}

pub enum ScannerError {
    MissingElement(Token, Position),
}

struct Scanner<'a> {
    current_position: Position,
    current_lexeme: String,
    source: MultiPeek<str::Chars<'a>>,
}

impl<'a> Scanner<'a> {
    fn initialize(source: &'a str) -> Scanner {
        Scanner {
            current_lexeme: "".into(),
            current_position: Position::initial(),
            source: multipeek(source.chars()),
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
            _ => unimplemented!(),
        };

        Some(result.map(|token| self.add_context(token, initial_position)))
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
