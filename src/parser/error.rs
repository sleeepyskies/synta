use thiserror::Error;

use crate::lexer::TokenKind;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParserError {
    #[error("expected an expression")]
    ExpectedExpression,

    #[error("expected a primary")]
    PrimaryExpected,

    #[error("expected a unary")]
    UnaryExpected,

    #[error("an unexpected token was encountered {0:?}")]
    UnexpectedToken(TokenKind),

    #[error("an lparen is missing a closing rparen")]
    UnclosedParen,

    #[error("expected a semicolon following an expression")]
    ExpectedSemicolon,

    #[error("expected an assignment following a declaration")]
    ExpectedAssignment,
}
