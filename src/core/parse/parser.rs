// TODO: remove this
#![allow(dead_code)]

use super::{
    super::{lexer::Lexer, token::Token},
    ast::Program,
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

    fn parse_program(&mut self) -> Option<Program> {
        None
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
}
