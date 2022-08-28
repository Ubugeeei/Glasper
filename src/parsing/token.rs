#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(PartialEq, Eq, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

pub fn lookup_indent(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _ => TokenType::IDENT,
    }
}
