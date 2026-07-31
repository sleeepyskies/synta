use std::{iter::Peekable, str::FromStr};

use thiserror::Error;

// TODO: better diagnostics here?

#[derive(Error, Debug, PartialEq)]
pub enum LexError {
    #[error("no keyword match")]
    NoKeywordMatch,
    #[error("unexpected character '{0}'")]
    UnexpectedCharacter(char),
    #[error("invalid numeric syntax '{0}'")]
    InvalidNumeric(String),
}

pub type LexResult<T> = ::std::result::Result<T, LexError>;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Set,
}

impl FromStr for Keyword {
    type Err = LexError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let keyword = match s {
            "set" => Self::Set,
            _ => Err(LexError::NoKeywordMatch)?,
        };
        Ok(keyword)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // single character tokens
    SemiColon,
    Colon,
    Assign,
    Plus,
    Minus,
    Slash,
    Asterisk,
    LParen,
    RParen,
    Bang,
    GreaterThan,
    LessThan,

    // dual character tokens
    Equals,
    NotEquals,
    GreaterEquals,
    LesserEquals,

    // literals
    Variable(String),
    Numeric(f64),
    Boolean(bool),
    Keyword(Keyword),
}

#[derive(Debug, PartialEq)]
pub struct TokenPosition {
    pub span: (usize, usize),
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub position: TokenPosition,
    pub lexeme: &'a str,
}

pub struct Lexer<'a> {
    source: &'a str,
    iterator: Peekable<std::str::Chars<'a>>,
    line: usize,
    column: usize,
    absolute_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            iterator: source.chars().peekable(),
            line: 0,
            column: 0,
            absolute_position: 0,
        }
    }

    /// Returns a copy of the next character, or None if there is no next character
    fn peek(&mut self) -> Option<char> {
        self.iterator.peek().copied()
    }

    /// Moves the cursor along by one character
    fn advance(&mut self) -> Option<char> {
        self.absolute_position += 1;
        self.column += 1;
        let mut next = self.iterator.next();
        // TODO: should add c.utf8_len() not 1 to byte offset
        match next {
            Some('\n') => {
                self.column = 0;
                self.line += 1;
            }
            Some('\r') if self.peek() == Some('\n') => {
                next = self.iterator.next();
                self.absolute_position += 1;
                self.column = 0;
                self.line += 1;
            }
            _ => (),
        }
        next
    }

    /// Keeps advancing the cursor until the next line in the
    /// source is reached. Mainly useful for handlilng line comments.
    fn advance_to_next_line(&mut self) {
        loop {
            // always returns \n even for \r\n
            if self.advance() == Some('\n') {
                return;
            }
        }
    }

    /// Keeps advancing the cursor as long as the following hold true:
    /// - The predicate returns true
    /// - EOF is not reached
    /// - The next parsed character is not a whitespace character
    ///
    /// Will return the first character that fails to satisfy one of these constraints.
    fn advance_while(&mut self, pred: fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if !pred(c) {
                break;
            }
            self.advance();
        }
    }

    fn create_lexeme(&self, start: usize) -> &'a str {
        &self.source[start..self.absolute_position]
    }

    fn create_token(&self, kind: TokenKind, start: usize) -> Token<'a> {
        Token {
            kind,
            position: TokenPosition {
                span: (
                    self.column - (self.absolute_position - start),
                    self.column - 1,
                ),
                line: self.line,
                column: self.column,
            },
            lexeme: self.create_lexeme(start),
        }
    }

    /// Parses any parse character based literal. This may be a boolean literal,
    /// variable or a synta keyword
    fn parse_character_based_literal(&mut self) -> LexResult<TokenKind> {
        let start = self.absolute_position - 1;

        self.advance_while(|c| c.is_ascii_alphanumeric() || c == '_');

        let end = self.absolute_position - 1;

        let word = &self.source[start..=end];

        match word {
            "true" => return Ok(TokenKind::Boolean(true)),
            "false" => return Ok(TokenKind::Boolean(false)),
            _ => (),
        }

        if let Ok(keyword) = Keyword::from_str(word) {
            return Ok(TokenKind::Keyword(keyword));
        }

        Ok(TokenKind::Variable(word.into()))
    }

    fn parse_numeric(&mut self) -> LexResult<TokenKind> {
        // TODO: this will allow any numbers allowed by f64::from_str, maybe this is too permissive?
        let start = self.absolute_position - 1;

        self.advance_while(|c| c.is_ascii_digit() || c == '.');

        let end = self.absolute_position - 1;

        match f64::from_str(&self.source[start..=end]) {
            Ok(num) => Ok(TokenKind::Numeric(num)),
            Err(_) => Err(LexError::InvalidNumeric(self.source[start..=end].into())),
        }
    }

    /// Reads over the source file until it can find the next relevant token.
    fn next_token(&mut self) -> LexResult<Option<Token<'a>>> {
        loop {
            self.advance_while(|c| c.is_whitespace());

            let start = self.absolute_position;

            let c = match self.advance() {
                None => return Ok(None),
                Some(c) => c,
            };

            let kind = match c {
                // single character tokens
                ';' => TokenKind::SemiColon,
                ':' => TokenKind::Colon,
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Asterisk,
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,

                // possible double character tokens
                '/' => match self.peek() {
                    Some('/') => {
                        self.advance();
                        self.advance_to_next_line();
                        continue;
                    }
                    _ => TokenKind::Slash,
                },
                '=' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        TokenKind::Equals
                    }
                    _ => TokenKind::Assign,
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        TokenKind::NotEquals
                    }
                    _ => TokenKind::Bang,
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        TokenKind::GreaterEquals
                    }
                    _ => TokenKind::GreaterThan,
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        TokenKind::LesserEquals
                    }
                    _ => TokenKind::LessThan,
                },

                '0'..='9' => self.parse_numeric()?,
                'a'..='z' | 'A'..='Z' => self.parse_character_based_literal()?,

                _ => return Err(LexError::UnexpectedCharacter(c)),
            };

            return Ok(Some(self.create_token(kind, start)));
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn lex_symbols() {
        let source = r#";:=+-/*()!!===//set 
            variable 167 
            hello 12.4
            >= <= < >
            true false
            "#;
        let lexer = Lexer::new(source);

        let expected = vec![
            TokenKind::SemiColon,
            TokenKind::Colon,
            TokenKind::Assign,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Slash,
            TokenKind::Asterisk,
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::Bang,
            TokenKind::NotEquals,
            TokenKind::Equals,
            TokenKind::Variable(String::from("variable")),
            TokenKind::Numeric(167f64),
            TokenKind::Variable(String::from("hello")),
            TokenKind::Numeric(12.4),
            TokenKind::GreaterEquals,
            TokenKind::LesserEquals,
            TokenKind::LessThan,
            TokenKind::GreaterThan,
            TokenKind::Boolean(true),
            TokenKind::Boolean(false),
        ];

        for (index, token) in lexer.enumerate() {
            let token = token.unwrap();
            println!("{:?}", token.kind);
            assert_eq!(token.kind, expected[index]);
        }
    }

    #[test]
    fn invalid_symbols() {
        let source = "@";
        let lexer = Lexer::new(source);

        for token in lexer {
            assert!(token.is_err());
        }
    }
}
