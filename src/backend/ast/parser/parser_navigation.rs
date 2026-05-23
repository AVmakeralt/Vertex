use crate::backend::lexer::tokens::{
        Token,
        TokenKind,
    };
use crate::backend::ast::parser::Parser;

pub trait ParserNavigation {
    fn current_token(&self) -> &Token;
    fn peek(&self) -> TokenKind;
    fn advance(&mut self);
}

impl ParserNavigation for Parser {
    fn current_token(&self) -> &Token {
        &self.tokens[self.token_idx]
    }

    fn advance(&mut self) {
        self.token_idx += 1;
    }

    fn peek(&self) -> TokenKind {
        let idx = self.token_idx + 1;
        self.tokens[idx].token_kind.clone()
    }
}

