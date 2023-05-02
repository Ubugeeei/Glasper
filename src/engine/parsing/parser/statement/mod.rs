pub mod block;
pub mod for_;
pub mod if_;
pub mod switch;
pub mod variables;

use std::io::Error;

use crate::engine::{
    ast::{Expression, Precedence, Statement},
    parsing::{lexer::token::TokenType, parser::Parser},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Const => self.parse_const_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::Switch => self.parse_switch_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::LBrace => self.parse_block_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_break_statement(&mut self) -> Result<Statement, Error> {
        self.next_token();
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token();
        }
        Ok(Statement::Break)
    }

    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        self.next_token();

        if self.cur_token.token_type == TokenType::SemiColon {
            return Ok(Statement::Return(Expression::Undefined));
        }

        let value: Expression = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token()
        }
        Ok(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token()
        }

        Ok(Statement::Expression(expr))
    }
}

#[cfg(test)]
mod test {
    use crate::engine::parsing::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_parse_return_statements() {
        {
            let source = String::from(
                r#"
                    return 5;
                    return 10;
                "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 2);
        }
    }
}
