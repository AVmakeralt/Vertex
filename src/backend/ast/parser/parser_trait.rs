pub struct Parser {
    tokens: Vec<Token>,
    token_idx: usize,
    on_top_statement: bool,
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
