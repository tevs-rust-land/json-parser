#[derive(Debug)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    SemiColon,
    NextLine,
    Whitespace,
    Comma,
    DigitLiteral(String),
    StringLiteral(String),
}

pub fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

pub fn is_alpha(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '.'
}

pub fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

pub fn is_nextline(c: char) -> bool {
    matches!(c, '\n')
}

pub fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t')
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn initial() -> Position {
        Position { line: 1, column: 1 }
    }

    pub fn increment_column(&mut self) {
        self.column += 1;
    }

    pub fn increment_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}
#[derive(Debug)]
pub struct TokenWithContext {
    pub token: Token,
    pub lexeme: String,
    pub position: Position,
}
