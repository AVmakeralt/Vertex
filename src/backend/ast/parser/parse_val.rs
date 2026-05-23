use crate::backend::{
    ast::nodes::BinaryOpNode,
    compiler::byte_code::Compilable,
    errors::parser_errors::ParserError::{self},
    lexer::tokens::TokenKind::{
            DIVIDE, EQUAL, GREATER, LESS, MINUS, MODULO, PLUS, TIMES,
        },
};

use crate::backend::ast::parser::Parser;
use crate::backend::ast::parser::*;

pub trait ParserDeclarations {
    fn parse_comparison(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
    fn parse_term(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
    fn parse_factor(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
}

impl ParserDeclarations for Parser {
    fn parse_comparison(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut factor = self.parse_term()?;
        while self.current_token().token_kind == GREATER
            || self.current_token().token_kind == LESS
            || self.current_token().token_kind == EQUAL
            {
                let operator = self.current_token().token_kind.clone();
                self.advance();
                factor = Box::new(BinaryOpNode {
                    left: factor,
                    right: self.parse_term()?,
                                    op_tok: operator,
                });
            }
            Ok(factor)
    }
    fn parse_term(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut factor = self.parse_factor()?;
        while self.current_token().token_kind == MINUS || self.current_token().token_kind == PLUS {
            let operator = self.current_token().token_kind.clone();
            self.advance();
            factor = Box::new(BinaryOpNode {
                left: factor,
                right: self.parse_factor()?,
                                op_tok: operator,
            });
        }
        Ok(factor)
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut factor = self.parse_unary()?;
        while self.current_token().token_kind == TIMES
            || self.current_token().token_kind == DIVIDE
            || self.current_token().token_kind == MODULO
            {
                let operator = self.current_token().token_kind.clone();
                self.advance();
                factor = Box::new(BinaryOpNode {
                    left: factor,
                    right: self.parse_unary()?,
                                    op_tok: operator,
                });
            }
            Ok(factor)
    }
}
