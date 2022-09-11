#![allow(non_camel_case_types)]

use crate::core::parse::ast::Precedence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident,
    Int,

    /*
     * symbols
     */
    Assign,
    // arithmetic operations
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    // comp
    LT,
    GT,
    // brackets
    LParen,
    RParen,
    LBrace,
    RBrace,
    // others
    Comma,
    SemiColon,

    /*
     * combination of symbols
     */
    Eq,
    NotEq,

    /*
     * keywords
     */
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }

    pub fn get_precedence(&mut self) -> Precedence {
        match self.token_type {
            TokenType::Eq => Precedence::Equals,
            TokenType::NotEq => Precedence::Equals,
            TokenType::LT => Precedence::LessGreater,
            TokenType::GT => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Slash => Precedence::Product,
            TokenType::Asterisk => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

pub fn lookup_indent(ident: &str) -> TokenType {
    match ident {
        "function" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}
