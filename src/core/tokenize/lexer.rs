use super::{token::lookup_indent, Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
        };
        l.read_char();
        l
    }

    #[allow(dead_code)]
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '\u{0}' => Token::new(TokenType::Eof, self.ch.to_string()),

            '+' => Token::new(TokenType::Plus, self.ch.to_string()),
            '-' => Token::new(TokenType::Minus, self.ch.to_string()),
            '*' => Token::new(TokenType::Asterisk, self.ch.to_string()),
            '/' => Token::new(TokenType::Slash, self.ch.to_string()),

            '<' => Token::new(TokenType::LT, self.ch.to_string()),
            '>' => Token::new(TokenType::GT, self.ch.to_string()),
            ';' => Token::new(TokenType::SemiColon, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '(' => Token::new(TokenType::LParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RParen, self.ch.to_string()),
            '{' => Token::new(TokenType::LBrace, self.ch.to_string()),
            '}' => Token::new(TokenType::RBrace, self.ch.to_string()),

            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==".to_string())
                } else {
                    Token::new(TokenType::Assign, self.ch.to_string())
                }
            }

            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, self.ch.to_string())
                }
            }

            _ => {
                if Self::is_letter(self.ch) {
                    let id = self.read_identifier();
                    let token_type = lookup_indent(&id);
                    Token::new(token_type, id)
                } else if Self::is_digit(self.ch) {
                    Token::new(TokenType::Int, self.read_number())
                } else {
                    Token::new(TokenType::Illegal, self.ch.to_string())
                }
            }
        };

        self.read_char();

        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Self::is_letter(self.ch) {
            self.read_char();
        }
        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\u{0}';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Self::is_digit(self.ch) {
            self.read_char();
        }
        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\u{0}'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    /*
     * static
     */

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_witespace() {
        let source = String::from(" \t\n\r=");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Assign);
    }

    #[test]
    fn test_digit() {
        let source = String::from("42;");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Int);
        assert_eq!(l.next_token().token_type, TokenType::SemiColon);
    }

    #[test]
    fn test_symbol_token() {
        let source = String::from("=+-*/!<>(){},;");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Assign);
        assert_eq!(l.next_token().token_type, TokenType::Plus);
        assert_eq!(l.next_token().token_type, TokenType::Minus);
        assert_eq!(l.next_token().token_type, TokenType::Asterisk);
        assert_eq!(l.next_token().token_type, TokenType::Slash);
        assert_eq!(l.next_token().token_type, TokenType::Bang);
        assert_eq!(l.next_token().token_type, TokenType::LT);
        assert_eq!(l.next_token().token_type, TokenType::GT);
        assert_eq!(l.next_token().token_type, TokenType::LParen);
        assert_eq!(l.next_token().token_type, TokenType::RParen);
        assert_eq!(l.next_token().token_type, TokenType::LBrace);
        assert_eq!(l.next_token().token_type, TokenType::RBrace);
        assert_eq!(l.next_token().token_type, TokenType::Comma);
        assert_eq!(l.next_token().token_type, TokenType::SemiColon);
    }

    #[test]
    fn test_combination_of_symbols() {
        let source = String::from("== !=");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Eq);
        assert_eq!(l.next_token().token_type, TokenType::NotEq);
    }

    #[test]
    fn test_keywords() {
        let source = String::from("function let true false if else return");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Function);
        assert_eq!(l.next_token().token_type, TokenType::Let);
        assert_eq!(l.next_token().token_type, TokenType::True);
        assert_eq!(l.next_token().token_type, TokenType::False);
        assert_eq!(l.next_token().token_type, TokenType::If);
        assert_eq!(l.next_token().token_type, TokenType::Else);
        assert_eq!(l.next_token().token_type, TokenType::Return);
    }

    #[test]
    fn test_tokenize() {
        let source = String::from(
            r#"
                let five = 5;
                let ten = 10;

                function zero() {
                    return 0;
                }

                let add = function(x, y) {
                    return x + y;
                };

                let result = add(five, ten);
            "#,
        );
        let mut l = Lexer::new(source);

        let mut t = l.next_token();
        assert_eq!(t.token_type, TokenType::Let);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("five"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Assign);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Int);
        assert_eq!(t.literal, String::from("5"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SemiColon);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Let);
        assert_eq!(t.literal, String::from("let"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("ten"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Assign);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Int);
        assert_eq!(t.literal, String::from("10"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SemiColon);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Function);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("zero"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LParen);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::RParen);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LBrace);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Return);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Int);
        assert_eq!(t.literal, String::from("0"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SemiColon);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::RBrace);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Let);
        assert_eq!(t.literal, String::from("let"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("add"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Assign);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Function);
        assert_eq!(t.literal, String::from("function"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LParen);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("x"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Comma);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("y"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::RParen);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LBrace);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Return);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("x"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Plus);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("y"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SemiColon);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::RBrace);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SemiColon);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Let);
        assert_eq!(t.literal, String::from("let"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("result"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Assign);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("add"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LParen);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("five"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Comma);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Ident);
        assert_eq!(t.literal, String::from("ten"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::RParen);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SemiColon);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Eof);
    }
}
