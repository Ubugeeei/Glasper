// TODO: remove this
#![allow(dead_code)]

use crate::core::tokenize::token::TokenType;

use super::{
    super::{lexer::Lexer, token::Token},
    ast::{Expression, Identifier, Program, Statement},
};

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peeked_token: Token,
}
impl<'a> Parser<'a> {
    fn new(l: &'a mut Lexer) -> Self {
        let first_token = l.next_token();
        let secound_token = l.next_token();

        Parser {
            l,
            cur_token: first_token,
            peeked_token: secound_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peeked_token.clone();
        self.peeked_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.cur_token.token_type != TokenType::Eof {
            let statements = self.parse_statement();
            program.statements.push(statements);
        }

        program
    }

    fn parse_statement(&mut self) -> Statement {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::If => self.parse_if_statement(),
            _ => todo!(),
        }
    }

    fn parse_let_statement(&mut self) -> Statement {
        todo!()
    }

    fn parse_return_statement(&mut self) -> Statement {
        todo!()
    }

    fn parse_if_statement(&mut self) -> Statement {
        todo!()
    }

    fn parse_identifier(&mut self) -> Identifier {
        todo!()
    }

    fn parse_expression(&mut self) -> Expression {
        todo!()
    }

    fn parse_operator_expression(&mut self) -> Expression {
        todo!()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::core::tokenize::token::TokenType;

    use super::*;

    #[test]
    fn test_new() {
        {
            let source = String::from("let five = 5;");
            let mut l = Lexer::new(source);
            let p = Parser::new(&mut l);

            assert_eq!(p.cur_token.token_type, TokenType::Let);
            assert_eq!(p.peeked_token.token_type, TokenType::Ident);
        }

        {
            let source = String::from("");
            let mut l = Lexer::new(source);
            let p = Parser::new(&mut l);

            assert_eq!(p.cur_token.token_type, TokenType::Eof);
            assert_eq!(p.peeked_token.token_type, TokenType::Eof);
        }
    }

    #[test]
    fn test_parse_let_statements() {
        let source = String::from(
            r#"
                let five = 5;
                let ten = 10;
        "#,
        );
        let mut l = Lexer::new(source);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 2);
        // TODO: test
    }
}
