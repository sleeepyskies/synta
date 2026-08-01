use thiserror::Error;

/// Errors that can occur while lexing source code.
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum LexError {
    /// A keyword was encountered that is not recognized by the language.
    #[error("unexpected keyword '{0}'")]
    UnexpectedKeyword(String),

    /// A character was encountered that has is disallowed by the lexer.
    #[error("unexpected character '{0}'")]
    UnexpectedCharacter(char),

    /// A numeric literal could not be parsed.
    #[error("invalid numeric syntax '{0}'")]
    InvalidNumeric(String),

    /// The lexer reached the end of the source unexpectedly.
    #[error("unexpected end of file")]
    UnexpectedEof,
}
