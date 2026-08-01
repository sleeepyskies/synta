use std::{ops::Range, str::FromStr};
use thiserror::Error;

/// A token produced by the lexer
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    /// The category of this token.
    pub kind: TokenKind,

    /// The location of this token in the source code.
    pub position: TokenPosition,

    /// The original source text represented by this token.
    pub lexeme: &'a str,
}

/// The different categories of tokens recognized by the lexer.
#[derive(Debug, PartialEq, Clone)]
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
    LogicalAnd,
    LogicalOr,

    // literals
    Variable(String),
    Numeric(f64),
    Boolean(bool),
    Keyword(Keyword),
}

/// A reserved keyword recognized by the lexer.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Keyword {
    /// The `set` keyword.
    Set,
}

/// The location of this token in the original source code.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenPosition {
    /// The byte range of this token in the original source code.
    pub span: Range<usize>,

    /// The line number where this token begins.
    pub line: usize,

    /// The column where this token begins.
    pub column: usize,
}

/// An error returned when a string could not match any of the reserved keywords.
#[derive(Error, Debug, PartialEq, Eq, Clone)]
#[error("could not parse keyword from '{0}'")]
pub struct ParseKeywordError(String);

impl FromStr for Keyword {
    type Err = ParseKeywordError;

    /// Parses a keyword from its source representation.
    ///
    /// Returns [`ParseKeywordError`] if the input is not a recognized keyword.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "set" => Ok(Self::Set),
            _ => Err(ParseKeywordError(s.to_owned())),
        }
    }
}
