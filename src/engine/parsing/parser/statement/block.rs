use std::io::{Error, ErrorKind};

use crate::engine::{
    ast::{BlockStatement, Statement},
    parsing::{lexer::token::TokenType, parser::Parser},
};

impl<'a> Parser<'a> {
    pub(in super::super) fn parse_block_statement(&mut self) -> Result<Statement, Error> {
        // guard
        if self.cur_token.token_type != TokenType::LBrace {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token '{{' but found {}",
                    self.peeked_token.literal
                ),
            ));
        }

        self.next_token(); // skip '{'

        let mut statements = vec![];
        while self.cur_token.token_type != TokenType::RBrace
            && self.cur_token.token_type != TokenType::Eof
        {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        Ok(Statement::Block(BlockStatement::new(statements)))
    }
}

#[cfg(test)]
mod test {
    // TODO: test
}
