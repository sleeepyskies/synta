mod expression;

pub use expression::Expr;

use crate::lexer::TokenKind;

struct Program {
    expressions: Vec<Expr>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            expressions: Vec::new(),
        }
    }
}

pub struct Parser {
    tokens: Vec<TokenKind>,
    index: usize,
    program: Program,
}

impl Parser {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self {
            tokens,
            index: 0,
            program: Program::new(),
        }
    }

    pub fn parse(&mut self) {
        while self.index < self.tokens.len() {
            self.expression();
        }
    }

    pub fn expressions(&self) -> &[Expr] {
        &self.program.expressions
    }

    fn peek(&self) -> Option<&TokenKind> {
        if self.index > self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.index])
        }
    }

    fn expression(&self) {
        match self.peek() {
            None => return,
            Some(kind) => println!("{kind:?}"),
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
