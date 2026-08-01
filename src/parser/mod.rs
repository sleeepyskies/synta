mod error;
mod expr;

pub use error::ParserError;
pub use expr::Expr;

use crate::{
    lexer::{Keyword, TokenKind},
    parser::expr::{BinaryOperator, UnaryOperator},
};

type ParserResult<T> = Result<T, ParserError>;

pub struct Parser {
    tokens: Vec<TokenKind>,
    index: usize,
}

impl Parser {
    pub const fn new(tokens: Vec<TokenKind>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> ParserResult<Vec<Expr>> {
        let mut expressions: Vec<Expr> = Vec::new();

        while !self.is_at_end() {
            let ex = self.expression()?;
            dbg!("{}", ex);
            expressions.push(self.expression()?);
        }

        Ok(expressions)
    }

    const fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn peek(&self) -> Option<TokenKind> {
        self.tokens.get(self.index).map(TokenKind::to_owned)
    }

    fn advance(&mut self) -> Option<TokenKind> {
        self.index += 1;
        self.peek()
    }

    fn expression(&mut self) -> ParserResult<Expr> {
        let expression = match self.peek() {
            Some(TokenKind::Keyword(Keyword::Set)) => self.declaration(),
            Some(_) => self.or(),
            None => Err(ParserError::ExpectedExpression),
        };

        if self.peek() != Some(TokenKind::SemiColon) {
            return Err(ParserError::ExpectedSemicolon);
        }

        expression
    }

    fn declaration(&mut self) -> ParserResult<Expr> {
        match (self.advance(), self.advance(), self.advance()) {
            (
                Some(TokenKind::Keyword(Keyword::Set)),
                Some(TokenKind::Variable(var)),
                Some(TokenKind::SemiColon),
            ) => Ok(Expr::Declaration {
                variable: var,
                assignment: None,
            }),
            (
                Some(TokenKind::Keyword(Keyword::Set)),
                Some(TokenKind::Variable(var)),
                Some(TokenKind::Assign),
            ) => {
                let assignment = self.expression()?;
                Ok(Expr::Declaration {
                    variable: var,
                    assignment: Some(Box::new(assignment)),
                })
            }
            _ => Err(ParserError::ExpectedAssignment),
        }
    }

    fn or(&mut self) -> ParserResult<Expr> {
        let left = self.and()?;

        let operator = match self.peek() {
            Some(TokenKind::Equals) => BinaryOperator::LogicalOr,
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Ok(left),
        };

        let right = self.and()?;

        Ok(Expr::BinOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn and(&mut self) -> ParserResult<Expr> {
        let left = self.equality()?;

        let operator = match self.peek() {
            Some(TokenKind::Equals) => BinaryOperator::LogicalAnd,
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Ok(left),
        };

        let right = self.equality()?;

        Ok(Expr::BinOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn equality(&mut self) -> ParserResult<Expr> {
        let left = self.comparison()?;

        let operator = match self.peek() {
            Some(TokenKind::Equals) => BinaryOperator::Equals,
            Some(TokenKind::NotEquals) => BinaryOperator::NotEquals,
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Ok(left),
        };

        let right = self.comparison()?;

        Ok(Expr::BinOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn comparison(&mut self) -> ParserResult<Expr> {
        let left = self.term()?;

        let operator = match self.peek() {
            Some(TokenKind::GreaterThan) => BinaryOperator::GreaterThan,
            Some(TokenKind::GreaterEquals) => BinaryOperator::GreaterEquals,
            Some(TokenKind::LessThan) => BinaryOperator::LesserThan,
            Some(TokenKind::LesserEquals) => BinaryOperator::LesserEquals,
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Ok(left),
        };

        let right = self.term()?;

        Ok(Expr::BinOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn term(&mut self) -> ParserResult<Expr> {
        let left = self.factor()?;

        let operator = match self.peek() {
            Some(TokenKind::Plus) => BinaryOperator::Plus,
            Some(TokenKind::Minus) => BinaryOperator::Minus,
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Ok(left),
        };

        let right = self.factor()?;

        Ok(Expr::BinOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn factor(&mut self) -> ParserResult<Expr> {
        let left = self.unary()?;

        let operator = match self.peek() {
            Some(TokenKind::Asterisk) => BinaryOperator::Times,
            Some(TokenKind::Slash) => BinaryOperator::Divide,
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Ok(left),
        };

        let right = self.unary()?;

        Ok(Expr::BinOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn unary(&mut self) -> ParserResult<Expr> {
        match self.peek() {
            Some(TokenKind::Bang) => {
                self.advance();
                Ok(Expr::UnOp {
                    operator: UnaryOperator::Not,
                    expression: Box::new(self.unary()?),
                })
            }
            Some(TokenKind::Minus) => {
                self.advance();
                Ok(Expr::UnOp {
                    operator: UnaryOperator::Minus,
                    expression: Box::new(self.unary()?),
                })
            }
            Some(_) => self.primary(),
            None => Err(ParserError::UnaryExpected),
        }
    }

    fn primary(&mut self) -> ParserResult<Expr> {
        match self.advance() {
            Some(TokenKind::Numeric(num)) => Ok(Expr::NumericLiteral(num)),
            Some(TokenKind::Boolean(b)) => Ok(Expr::BooleanLiteral(b)),
            Some(TokenKind::Variable(var)) => Ok(Expr::Variable(var)),

            Some(TokenKind::LParen) => {
                self.advance();
                let expression = self.expression()?;
                if self.peek() == Some(TokenKind::RParen) {
                    return Ok(expression);
                }
                Err(ParserError::UnclosedParen)
            }
            Some(token) => Err(ParserError::UnexpectedToken(token)),
            None => Err(ParserError::PrimaryExpected),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Keyword, TokenKind};
    use crate::parser::*;

    #[test]
    fn blabla() {
        let tokens = vec![TokenKind::Keyword(Keyword::Set)];
        let _parser = Parser::new(tokens);
    }
}
