#![allow(non_camel_case_types)]

use crate::engine::ast::Precedence;

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
    String,
    NaN,

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
    BitNot,
    // comp
    Lt,
    Gt,
    Lte,
    Gte,
    // brackets
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    // bool
    Or,
    And,
    Conditional, // ?
    // others
    Comma,
    Colon,
    SemiColon,
    Period,

    /*
     * combination of symbols
     */
    Eq,
    NotEq,
    EqStrict,
    NotEqStrict,
    Inc,
    Dec,
    Exp,
    NullishCoalescing, // ??
    ShL,
    ShR,
    SaR,
    Typeof,

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
    Switch,
    Case,
    Default,
    For,
    Continue,
    Break,
    Null,
    Undefined,
    This,
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
            TokenType::Eq | TokenType::EqStrict | TokenType::NotEqStrict => Precedence::Equals,
            TokenType::NullishCoalescing => Precedence::NullishCoalescing,
            TokenType::Or | TokenType::And => Precedence::Bool,
            TokenType::NotEq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt | TokenType::Lte | TokenType::Gte => {
                Precedence::LessGreater
            }
            TokenType::BitOr | TokenType::BitAnd | TokenType::BitXOr => Precedence::Sum,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Exp => Precedence::Exp,
            TokenType::ShL | TokenType::ShR | TokenType::SaR => Precedence::Shift,
            TokenType::Slash | TokenType::Asterisk | TokenType::Percent => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            TokenType::LBracket => Precedence::Index,
            TokenType::Period => Precedence::Index,
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
        "switch" => TokenType::Switch,
        "case" => TokenType::Case,
        "default" => TokenType::Default,
        "for" => TokenType::For,
        "continue" => TokenType::Continue,
        "break" => TokenType::Break,
        "return" => TokenType::Return,
        "null" => TokenType::Null,
        "undefined" => TokenType::Undefined,
        "typeof" => TokenType::Typeof,
        "NaN" => TokenType::NaN,
        "this" => TokenType::This,
        _ => TokenType::Ident,
    }
}
