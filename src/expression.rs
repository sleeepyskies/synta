use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Divide,
    Times,
    Equals,
    NotEquals,
    GreaterThan,
    LesserThan,
    GreaterEquals,
    LesserEquals,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Delcaration {
        variable: String,
        assignment: Option<Box<Expression>>,
    },
    BinOp {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnOp {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    Grouping {
        inner: Box<Expression>,
    },
    NumericLiteral(f64),
    BooleanLiteral(bool),
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Divide => "/",
            BinaryOperator::Times => "*",
            BinaryOperator::Equals => "==",
            BinaryOperator::NotEquals => "!=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::LesserThan => "<",
            BinaryOperator::GreaterEquals => ">=",
            BinaryOperator::LesserEquals => "<=",
        })
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnaryOperator::Not => "!",
            UnaryOperator::Minus => "-",
        })
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Delcaration {
                variable,
                assignment,
            } => match assignment {
                Some(e) => write!(f, "(set {variable} {e})"),
                None => write!(f, "(set {variable})"),
            },
            Expression::BinOp {
                operator,
                left,
                right,
            } => write!(f, "({operator} {left} {right})"),
            Expression::UnOp {
                operator,
                expression,
            } => write!(f, "({operator} {expression})"),
            Expression::Grouping { inner } => write!(f, "({inner})"),
            Expression::NumericLiteral(value) => write!(f, "{value}"),
            Expression::BooleanLiteral(value) => write!(f, "{value}"),
        }
    }
}
