use crate::backend::{
    ast::{
        nodes::{
            BinaryOpNode, BoolNode,
            CallType::{Fn, Macro},
            FloatNode, FunctionCallNode, ImportNode, LoopNode, NumberNode, ProgramNode, ReturnNode,
            StringNode, VariableAccessNode, VariableAssignNode, VariableDefineNode,
        },
        statements::{if_statement::IfStatement, while_statement::WhileStatement},
    },
    compiler::byte_code::Compilable,
    errors::parser_errors::ParserError::{self, UnexpectedToken},
    lexer::tokens::{
        Token,
        TokenKind::{
            self, ASSIGN, CLOSINGBRACE, COLON, COMMA, CONST, DIVIDE, ELSE, EOF, EQUAL, FALSE,
            FLOAT, FNC, GREATER, IDENTIFIER, IF, LEFTPAREN, LESS, MINUS, MODULO, NUMB,
            OPENINGBRACE, PLUS, RIGHTPAREN, SEMICOLON, STRING, TIMES, TRUE, USE, VALUE, VAR, WHILE,
        },
    },
};

use crate::backend::ast::functions::{args_node::FunctionArgs, function_nodes::FunctionDefineNode};

use crate::backend::ast::parser::Parser;
use crate::backend::ast::parser::*;

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

