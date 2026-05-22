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
use crate::backend::ast::parser::parser_navigation::ParserNavigation;
use crate::backend::ast::parser::parse_stmt::ParseStatments;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub token_idx: usize,
    pub on_top_statement: bool,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        Self {
            tokens: token_list,
            token_idx: 0,
            on_top_statement: true,
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        let mut program: ProgramNode = ProgramNode::new();
        while self.on_top_statement && self.current_token().token_kind != EOF {
            program.program_nodes.push(self.parse_top_statement()?);
        }

        while self.current_token().token_kind != EOF {
            program.program_nodes.push(self.parse_stmt()?)
        }
        Ok(Box::new(program))
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<Token, ParserError> {
        if self.current_token().token_kind == token_kind {
            let token = self.current_token().clone();
            self.advance();
            Ok(token)
        } else {
            Err(UnexpectedToken {
                expected: token_kind,
                found: self.current_token().token_value.clone(),
            })
        }
    }
}
