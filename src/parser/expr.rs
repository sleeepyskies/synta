use std::fmt::Display;
use crate::lexer::TokenKind;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Divide,
    Times,

    LogicalAnd,
    LogicalOr,

    Equals,
    NotEquals,
    GreaterThan,
    LesserThan,
    GreaterEquals,
    LesserEquals,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum UnaryOperator {
    Not,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Declaration {
        variable: String,
        assignment: Option<Box<Self>>,
    },
    BinOp {
        operator: BinaryOperator,
        left: Box<Self>,
        right: Box<Self>,
    },
    UnOp {
        operator: UnaryOperator,
        expression: Box<Self>,
    },
    NumericLiteral(f64),
    BooleanLiteral(bool),
    Variable(String),
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Divide => "/",
                Self::Times => "*",
                Self::LogicalAnd => "&&",
                Self::LogicalOr => "||",
                Self::Equals => "==",
                Self::NotEquals => "!=",
                Self::GreaterThan => ">",
                Self::LesserThan => "<",
                Self::GreaterEquals => ">=",
                Self::LesserEquals => "<=",
            }
        )
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Not => "!",
            Self::Minus => "-",
        })
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Declaration {
                variable,
                assignment,
            } => match assignment {
                Some(e) => write!(f, "(set {variable} = {e})"),
                None => write!(f, "(set {variable})"),
            },
            Self::BinOp {
                operator,
                left,
                right,
            } => write!(f, "({operator} {left} {right})"),
            Self::UnOp {
                operator,
                expression,
            } => write!(f, "({operator} {expression})"),
            Self::NumericLiteral(value) => write!(f, "{value}"),
            Self::BooleanLiteral(value) => write!(f, "{value}"),
            Self::Variable(var) => write!(f, "{var}"),
        }
    }
}

impl TryFrom<TokenKind> for BinaryOperator {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Minus => Ok(Self::Minus),
            TokenKind::Plus => Ok(Self::Plus),
            TokenKind::Slash => Ok(Self::Divide),
            TokenKind::Asterisk => Ok(Self::Times),
            TokenKind::GreaterThan => Ok(Self::GreaterThan),
            TokenKind::LessThan => Ok(Self::LesserThan),
            TokenKind::Equals => Ok(Self::Equals),
            TokenKind::NotEquals => Ok(Self::NotEquals),
            TokenKind::GreaterEquals => Ok(Self::GreaterEquals),
            TokenKind::LesserEquals => Ok(Self::LesserEquals),
            TokenKind::LogicalAnd => Ok(Self::LogicalAnd),
            TokenKind::LogicalOr => Ok(Self::LogicalOr),
            _ => Err(()),
        }
    }
}

impl TryFrom<TokenKind> for UnaryOperator {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Minus => Ok(Self::Minus),
            TokenKind::Bang => Ok(Self::Not),
            _ => Err(()),
        }
    }
}
