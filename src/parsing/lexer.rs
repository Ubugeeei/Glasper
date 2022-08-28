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
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '\u{0}' => Token::new(TokenType::EOF, self.ch.to_string()),

            '+' => Token::new(TokenType::PLUS, self.ch.to_string()),
            '-' => Token::new(TokenType::MINUS, self.ch.to_string()),
            '*' => Token::new(TokenType::ASTERISK, self.ch.to_string()),
            '/' => Token::new(TokenType::SLASH, self.ch.to_string()),

            '<' => Token::new(TokenType::LT, self.ch.to_string()),
            '>' => Token::new(TokenType::GT, self.ch.to_string()),
            ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
            '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
            '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),

            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::EQ, self.ch.to_string())
                } else {
                    Token::new(TokenType::ASSIGN, self.ch.to_string())
                }
            }

            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NOT_EQ, self.ch.to_string())
                } else {
                    Token::new(TokenType::BANG, self.ch.to_string())
                }
            }

            _ => {
                if self.is_letter() {
                    let id = self.read_identifier();
                    let token_type = lookup_indent(&id);
                    Token::new(token_type, id)
                } else if self.is_digit() {
                    Token::new(TokenType::INT, self.read_number())
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch.to_string())
                }
            }
        };

        self.read_char();

        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
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
        while self.is_digit() {
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

    fn is_letter(&self) -> bool {
        self.ch.is_alphabetic() || self.ch == '_'
    }

    fn is_digit(&self) -> bool {
        self.ch.is_ascii_digit()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_witespace() {
        let source = String::from(" \t\n\r=");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::ASSIGN);
    }

    #[test]
    fn test_digit() {
        let source = String::from("42;");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::INT);
        assert_eq!(l.next_token().token_type, TokenType::SEMICOLON);
    }

    #[test]
    fn test_symbol_token() {
        let source = String::from("=+-*/!<>(){},;");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::ASSIGN);
        assert_eq!(l.next_token().token_type, TokenType::PLUS);
        assert_eq!(l.next_token().token_type, TokenType::MINUS);
        assert_eq!(l.next_token().token_type, TokenType::ASTERISK);
        assert_eq!(l.next_token().token_type, TokenType::SLASH);
        assert_eq!(l.next_token().token_type, TokenType::BANG);
        assert_eq!(l.next_token().token_type, TokenType::LT);
        assert_eq!(l.next_token().token_type, TokenType::GT);
        assert_eq!(l.next_token().token_type, TokenType::LPAREN);
        assert_eq!(l.next_token().token_type, TokenType::RPAREN);
        assert_eq!(l.next_token().token_type, TokenType::LBRACE);
        assert_eq!(l.next_token().token_type, TokenType::RBRACE);
        assert_eq!(l.next_token().token_type, TokenType::COMMA);
        assert_eq!(l.next_token().token_type, TokenType::SEMICOLON);
    }

    #[test]
    fn test_combination_of_symbols() {
        let source = String::from("== !=");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::EQ);
        assert_eq!(l.next_token().token_type, TokenType::NOT_EQ);
    }

    #[test]
    fn test_keywords() {
        let source = String::from("fn let");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::FUNCTION);
        assert_eq!(l.next_token().token_type, TokenType::LET);
    }

    #[test]
    fn test_tokenize() {
        let source = String::from(
            r#"
                let five = 5;
                let ten = 10;

                let add = fn(x, y) {
                    x + y;
                };

                let result = add(five, ten);
            "#,
        );
        let mut l = Lexer::new(source);

        let mut t = l.next_token();
        assert_eq!(t.token_type, TokenType::LET);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("five"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::ASSIGN);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::INT);
        assert_eq!(t.literal, String::from("5"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SEMICOLON);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LET);
        assert_eq!(t.literal, String::from("let"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("ten"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::ASSIGN);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::INT);
        assert_eq!(t.literal, String::from("10"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SEMICOLON);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LET);
        assert_eq!(t.literal, String::from("let"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("add"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::ASSIGN);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::FUNCTION);
        assert_eq!(t.literal, String::from("fn"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LPAREN);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("x"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::COMMA);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("y"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::RPAREN);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LBRACE);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("x"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::PLUS);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("y"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SEMICOLON);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::RBRACE);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SEMICOLON);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LET);
        assert_eq!(t.literal, String::from("let"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("result"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::ASSIGN);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("add"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::LPAREN);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("five"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::COMMA);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::IDENT);
        assert_eq!(t.literal, String::from("ten"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::RPAREN);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SEMICOLON);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::EOF);
    }
}
