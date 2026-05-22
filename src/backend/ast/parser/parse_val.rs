pub trait ParserDeclarations {
    fn parse_var_decl_stmt(&mut self, is_pub: bool) -> Result<Box<dyn Compilable>, ParserError>;
    fn parse_comparison(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
    fn parse_term(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
    fn parse_factor(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
}

impl ParserDeclarations for Parser {
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
