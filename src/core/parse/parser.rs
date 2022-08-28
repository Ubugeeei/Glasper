// TODO: remove this
#![allow(dead_code)]

use crate::core::tokenize::{lexer::Lexer, token::Token};

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
}
