pub trait ParseUnary {
    fn parse_unary(&mut self) -> Result<Box<dyn Compilable>, ParserError>;
}

impl ParseUnary for Parser {
    fn parse_unary(&mut self) -> Result<Box<dyn Compilable>, ParserError> {
        if self.current_token().token_kind == FLOAT {
            let value = match self.current_token().token_value.parse::<f32>() {
                Err(_) => unreachable!(),
                Ok(numb) => numb,
            };
            self.advance();
            Ok(Box::new(FloatNode { number: value }))
        } else if self.current_token().token_kind == TRUE
            || self.current_token().token_kind == FALSE
            {
                let value = self.current_token().token_kind.clone();
                self.advance();
                Ok(Box::new(BoolNode { value }))
            } else if self.current_token().token_kind == NUMB {
                let value = match self.current_token().token_value.parse::<i64>() {
                    Ok(numb) => numb,
                    Err(_) => unreachable!(),
                };
                self.advance();
                Ok(Box::new(NumberNode { number: value }))
            } else if self.current_token().token_kind == IDENTIFIER {
                let value = self.current_token().token_value.clone();
                self.advance();

                if self.current_token().token_kind == LEFTPAREN {
                    self.advance();
                    let mut args: Vec<Box<dyn Compilable>> = Vec::new();

                    if self.current_token().token_kind != RIGHTPAREN {
                        loop {
                            args.push(self.parse_expr()?);

                            if self.current_token().token_kind == COMMA {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }

                    self.expect(RIGHTPAREN)?;
                    let is_macro = value.ends_with('!');
                    let name = value.trim_end_matches('!').to_string();

                    Ok(Box::new(FunctionCallNode {
                        args,
                        name,
                        call_type: if is_macro { Macro } else { Fn },
                        return_type: None,
                    }))
                } else {
                    Ok(Box::new(VariableAccessNode {
                        variable_name: value,
                    }))
                }
            } else if self.current_token().token_kind == LEFTPAREN {
                self.advance();
                let value = self.parse_expr()?;
                self.expect(RIGHTPAREN)?;
                Ok(value)
            } else if self.current_token().token_kind == STRING {
                let value = StringNode {
                    value: self.current_token().token_value.clone(),
                };
                self.advance();
                Ok(Box::new(value))
            } else {
                Err(UnexpectedToken {
                    found: self.current_token().token_value.clone(),
                    expected: VALUE,
                })
            }
    }
}
