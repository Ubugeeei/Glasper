#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEAGAL,
    EOF,

    IDENT(String),
    INT(f64),

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}
