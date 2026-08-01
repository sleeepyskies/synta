mod error;
mod token;

pub use error::LexError;
pub use token::{Keyword, Token, TokenKind, TokenPosition};

use std::{iter::Peekable, str::FromStr};

// TODO: better diagnostics here?

pub type LexResult<T> = Result<T, LexError>;

pub struct Lexer<'a> {
    source: &'a str,
    iterator: Peekable<std::str::Chars<'a>>,
    line: usize,
    column: usize,
    byte_offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            iterator: source.chars().peekable(),
            line: 0,
            column: 0,
            byte_offset: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.iterator.peek().copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.iterator.next()?;

        self.byte_offset += c.len_utf8();
        self.column += 1;

        match c {
            '\n' => {
                self.column = 0;
                self.line += 1;
            }
            '\r' if self.peek() == Some('\n') => {
                self.iterator.next();
                self.byte_offset += 1;
                self.column = 0;
                self.line += 1;
                return Some('\n');
            }
            _ => {}
        }

        Some(c)
    }

    fn advance_while<F>(&mut self, pred: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(c) = self.peek() {
            if !pred(c) {
                break;
            }
            self.advance();
        }
    }

    fn skip_whitespace(&mut self) {
        self.advance_while(char::is_whitespace);
    }

    fn next_line(&mut self) {
        self.advance_while(|c| c != '\n');
    }

    fn lexeme(&self, start: usize) -> &'a str {
        &self.source[start..self.byte_offset]
    }

    fn token_from(&self, kind: TokenKind, start: usize) -> Token<'a> {
        Token {
            kind,
            position: TokenPosition {
                span: start..self.byte_offset,
                line: self.line,
                column: self.column,
            },
            lexeme: self.lexeme(start),
        }
    }

    fn character_sequence(&mut self, start: usize) -> TokenKind {
        self.advance_while(|c| c.is_ascii_alphanumeric() || c == '_');

        let word = self.lexeme(start);

        match word {
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            _ => {
                if let Ok(keyword) = Keyword::from_str(word) {
                    return TokenKind::Keyword(keyword);
                }
                TokenKind::Variable(word.to_owned())
            }
        }
    }

    fn numeric(&mut self, start: usize) -> LexResult<TokenKind> {
        self.advance_while(|c| c.is_ascii_digit() || c == '.');

        f64::from_str(self.lexeme(start)).map_or_else(
            |_| Err(LexError::InvalidNumeric(self.lexeme(start).to_owned())),
            |num| Ok(TokenKind::Numeric(num)),
        )
    }

    fn next_token(&mut self) -> LexResult<Option<Token<'a>>> {
        loop {
            self.skip_whitespace();
            let start = self.byte_offset;

            let Some(c) = self.advance() else {
                return Ok(None);
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
                        self.next_line();
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
                '&' => match self.peek() {
                    Some('&') => {
                        self.advance();
                        TokenKind::LogicalAnd
                    }
                    Some(c) => return Err(LexError::UnexpectedCharacter(c)),
                    None => return Err(LexError::UnexpectedEof),
                },
                '|' => match self.peek() {
                    Some('|') => {
                        self.advance();
                        TokenKind::LogicalOr
                    }
                    Some(c) => return Err(LexError::UnexpectedCharacter(c)),
                    None => return Err(LexError::UnexpectedEof),
                },

                '0'..='9' => self.numeric(start)?,
                'a'..='z' | 'A'..='Z' => self.character_sequence(start),

                _ => return Err(LexError::UnexpectedCharacter(c)),
            };

            return Ok(Some(self.token_from(kind, start)));
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
        let source = ";:=+-/*()!!===//set\nvariable 167\nhello 12.4\n>= <= < >\ntrue false\n";
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

        let tokens: Vec<TokenKind> = lexer.map(|t| t.unwrap().kind).collect();
        assert_eq!(expected, tokens);
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
