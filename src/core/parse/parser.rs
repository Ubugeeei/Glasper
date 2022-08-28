// TODO: remove this
#![allow(dead_code)]

use crate::core::tokenize::{
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    l: Lexer,
    cur_tok: Token,
    peek_tok: Option<Token>,
}
impl Parser {
    fn new(l: Lexer) -> Self {
        Parser {
            l,
            cur_tok: Token::new(TokenType::InitialValue, "".to_string()),
            peek_tok: None,
        }
    }
}
