use std::fmt::Display;

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
    Grouping {
        inner: Box<Self>,
    },
    NumericLiteral(f64),
    BooleanLiteral(bool),
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
            Self::Grouping { inner } => write!(f, "({inner})"),
            Self::NumericLiteral(value) => write!(f, "{value}"),
            Self::BooleanLiteral(value) => write!(f, "{value}"),
        }
    }
}
