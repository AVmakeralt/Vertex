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

pub trait ParseStatments {
    fn parse_top_statement(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
    fn parse_stmt(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
    fn parse_var_decl_stmt(&mut self, is_pub: bool) -> Result<Box<dyn Compilable>, ParserError>;
}

impl ParseStatments for Parser {
    fn parse_top_statement(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        match self.current_token().token_kind {
            USE => {
                self.advance();
                let name_to_use = self.expect(STRING)?.token_value;
                self.expect(SEMICOLON)?;
                Ok(Box::new(ImportNode {
                    module: name_to_use,
                }))
            }
            _ => {
                self.on_top_statement = false;
                self.parse_stmt()
            }
        }
    }

    fn parse_stmt(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        match &self.current_token().token_kind {
            TokenKind::LOOP => {
                self.advance();
                self.expect(OPENINGBRACE)?;
                let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                while self.current_token().token_kind != CLOSINGBRACE {
                    if self.current_token().token_kind == EOF {
                        return Err(ParserError::UnexpectedToken {
                            found: "EOF".into(),
                                expected: SEMICOLON,
                        });
                    }
                    body.push(self.parse_stmt()?);
                }
                self.expect(CLOSINGBRACE)?;
                Ok(Box::new(LoopNode { body }))
            }
            TokenKind::RETURN => {
                self.advance();
                if self.current_token().token_kind == SEMICOLON {
                    self.expect(SEMICOLON)?;
                    Ok(Box::new(ReturnNode { returns: None }))
                } else {
                    let value = self.parse_expr()?;
                    self.expect(SEMICOLON)?;
                    Ok(Box::new(ReturnNode {
                        returns: Some(value),
                    }))
                }
            }
            TokenKind::EXP => {
                self.advance();
                if self.current_token().token_kind == CONST
                    || self.current_token().token_kind == VAR
                    {
                        let value = self.parse_var_decl_stmt(true)?;
                        self.expect(SEMICOLON)?;
                        Ok(value)
                    } else if self.current_token().token_kind == FNC {
                        let mut args = Vec::new();
                        self.advance(); //FN
                        let id = self.expect(IDENTIFIER)?;
                        self.expect(LEFTPAREN)?;
                        if self.current_token().token_kind != RIGHTPAREN {
                            loop {
                                let arg_name = self.expect(IDENTIFIER)?;
                                self.expect(COLON)?;
                                let arg_type = self.expect(IDENTIFIER)?;

                                args.push(FunctionArgs {
                                    name: arg_name.token_value,
                                    argument_type: arg_type.token_value,
                                });

                                if self.current_token().token_kind == COMMA {
                                    self.advance();
                                    continue;
                                }

                                break;
                            }
                        }
                        self.expect(RIGHTPAREN)?;
                        let return_type = if self.current_token().token_kind == COLON {
                            self.advance();
                            Some(self.expect(IDENTIFIER)?.token_value)
                        } else {
                            None
                        };
                        self.expect(OPENINGBRACE)?;

                        let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                        while self.current_token().token_kind != CLOSINGBRACE {
                            body.push(self.parse_stmt()?);
                        }
                        self.expect(CLOSINGBRACE)?;

                        Ok(Box::new(FunctionDefineNode {
                            id: id.token_value,
                            return_type,
                            body,
                            args,
                        }))
                    } else {
                        Err(UnexpectedToken {
                            expected: VAR,
                            found: self.current_token().token_value.clone(),
                        })
                    }
            }
            VAR | CONST => {
                let value = self.parse_var_decl_stmt(false);
                self.expect(SEMICOLON)?;
                value
            }
            IDENTIFIER if self.peek() == ASSIGN => {
                let id = self.current_token().token_value.clone();
                self.advance();
                self.expect(ASSIGN)?;
                let value = self.parse_expr()?;
                self.expect(SEMICOLON)?;
                Ok(Box::new(VariableAssignNode { name: id, value }))
            }
            IF => {
                self.advance();
                self.expect(LEFTPAREN)?;
                let condition = self.parse_expr()?;
                self.expect(RIGHTPAREN)?;
                self.expect(OPENINGBRACE)?;
                let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                while self.current_token().token_kind != CLOSINGBRACE {
                    if self.current_token().token_kind == EOF {
                        return Err(ParserError::UnexpectedToken {
                            found: "EOF".into(),
                                expected: SEMICOLON,
                        });
                    }
                    body.push(self.parse_stmt()?);
                }
                self.expect(CLOSINGBRACE)?;
                if self.current_token().token_kind == ELSE {
                    self.advance();
                    self.expect(OPENINGBRACE)?;
                    let mut else_body: Vec<Box<dyn Compilable>> = Vec::new();
                    while self.current_token().token_kind != CLOSINGBRACE {
                        if self.current_token().token_kind == EOF {
                            return Err(ParserError::UnexpectedToken {
                                found: "EOF".into(),
                                    expected: SEMICOLON,
                            });
                        }
                        else_body.push(self.parse_stmt()?);
                    }
                    self.expect(CLOSINGBRACE)?;
                    return Ok(Box::new(IfStatement {
                        condition,
                        then_branch: body,
                        else_branch: Some(else_body),
                    }));
                }
                Ok(Box::new(IfStatement {
                    condition,
                    then_branch: body,
                    else_branch: None,
                }))
            }
            WHILE => {
                self.advance();
                self.expect(LEFTPAREN)?;
                let condition = self.parse_expr()?;
                self.expect(RIGHTPAREN)?;
                self.expect(OPENINGBRACE)?;
                let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                while self.current_token().token_kind != CLOSINGBRACE {
                    if self.current_token().token_kind == EOF {
                        return Err(ParserError::UnexpectedToken {
                            found: "EOF".into(),
                                expected: SEMICOLON,
                        });
                    }
                    body.push(self.parse_stmt()?);
                }
                self.expect(CLOSINGBRACE)?;
                Ok(Box::new(WhileStatement { condition, body }))
            }
            FNC => {
                let mut args = Vec::new();
                self.advance(); //FN
                let id = self.expect(IDENTIFIER)?;
                self.expect(LEFTPAREN)?;
                if self.current_token().token_kind != RIGHTPAREN {
                    loop {
                        let arg_name = self.expect(IDENTIFIER)?;
                        self.expect(COLON)?;
                        let arg_type = self.expect(IDENTIFIER)?;

                        args.push(FunctionArgs {
                            name: arg_name.token_value,
                            argument_type: arg_type.token_value,
                        });

                        if self.current_token().token_kind == COMMA {
                            self.advance();
                            continue;
                        }

                        break;
                    }
                }
                self.expect(RIGHTPAREN)?;
                let reurn_type = if self.current_token().token_kind == COLON {
                    self.advance();
                    Some(self.expect(IDENTIFIER)?.token_value)
                } else {
                    None
                };
                self.expect(OPENINGBRACE)?;

                let mut body: Vec<Box<dyn Compilable>> = Vec::new();
                while self.current_token().token_kind != CLOSINGBRACE {
                    body.push(self.parse_stmt()?);
                }
                self.expect(CLOSINGBRACE)?;

                Ok(Box::new(FunctionDefineNode {
                    id: id.token_value,
                    return_type: reurn_type,
                    body,
                    args,
                }))
            }
            _ => {
                let expr = self.parse_expr();
                self.expect(SEMICOLON)?;
                expr
            }
        }
    }

    fn parse_var_decl_stmt(&mut self, is_pub: bool) -> Result<Box<dyn Compilable>, ParserError> {
        let is_const: bool = self.current_token().token_kind == CONST;

        self.advance();
        let id: String = self.expect(IDENTIFIER)?.token_value;
        let mut value_type = None;

        if self.current_token().token_kind == COLON {
            self.advance();
            value_type = Some(self.expect(IDENTIFIER)?.token_value);
        }

        let value: Option<Box<dyn Compilable>> = if self.current_token().token_kind == ASSIGN {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(Box::new(VariableDefineNode {
            value_type,
            value,
            var_name: id,
            is_const,
            is_public: is_pub,
        }))
    }
}
