#![allow(non_camel_case_types)]

use crate::engine::parse::ast::Precedence;

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
    Number,

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
    Percent,
    BitOr,
    BitAnd,
    BitXOr,
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
    // bool
    Or,
    And,
    Conditional, // ?

    /*
     * combination of symbols
     */
    Eq,
    NotEq,
    Inc,
    Dec,
    Exp,
    NullishCoalescing, // ??
    ShL,
    ShR,
    SaR,

    /*
     * keywords
     */
    Function,
    Let,
    Const,
    True,
    False,
    If,
    Else,
    Return,
    Null,
    Undefined,
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
            TokenType::Assign => Precedence::Assign,
            TokenType::Eq => Precedence::Equals,
            TokenType::NullishCoalescing => Precedence::NullishCoalescing,
            TokenType::Or | TokenType::And => Precedence::Bool,
            TokenType::NotEq => Precedence::Equals,
            TokenType::LT | TokenType::GT => Precedence::LessGreater,
            TokenType::BitOr | TokenType::BitAnd | TokenType::BitXOr => Precedence::Sum,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Exp => Precedence::Exp,
            TokenType::ShL | TokenType::ShR | TokenType::SaR => Precedence::Shift,
            TokenType::Slash | TokenType::Asterisk | TokenType::Percent => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

pub fn lookup_indent(ident: &str) -> TokenType {
    match ident {
        "function" => TokenType::Function,
        "let" => TokenType::Let,
        "const" => TokenType::Const,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        "null" => TokenType::Null,
        "undefined" => TokenType::Undefined,
        _ => TokenType::Ident,
    }
}
