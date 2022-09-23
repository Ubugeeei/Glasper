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

            '+' => {
                if self.peek_char() == '+' {
                    self.read_char();
                    Token::new(TokenType::Inc, "++".to_string())
                } else {
                    Token::new(TokenType::Plus, self.ch.to_string())
                }
            }
            '-' => {
                if self.peek_char() == '-' {
                    self.read_char();
                    Token::new(TokenType::Dec, "--".to_string())
                } else {
                    Token::new(TokenType::Minus, self.ch.to_string())
                }
            }
            '*' => {
                if self.peek_char() == '*' {
                    self.read_char();
                    Token::new(TokenType::Exp, "**".to_string())
                } else {
                    Token::new(TokenType::Asterisk, self.ch.to_string())
                }
            }
            '/' => {
                match self.peek_char() {
                    // skip line comment
                    '/' => {
                        self.read_char();
                        loop {
                            self.read_char();
                            if self.ch == '\n' {
                                self.read_char();
                                break self.next_token();
                            }
                        }
                    }

                    // skip block comment
                    '*' => {
                        self.read_char();
                        loop {
                            self.read_char();
                            if self.ch == '*' && self.peek_char() == '/' {
                                self.read_char();
                                self.read_char();
                                break self.next_token();
                            }
                        }
                    }

                    _ => Token::new(TokenType::Slash, self.ch.to_string()),
                }
            }
            '%' => Token::new(TokenType::Percent, self.ch.to_string()),

            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    Token::new(TokenType::Or, "||".to_string())
                } else {
                    Token::new(TokenType::BitOr, self.ch.to_string())
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    Token::new(TokenType::And, "&&".to_string())
                } else {
                    Token::new(TokenType::BitAnd, self.ch.to_string())
                }
            }
            '^' => Token::new(TokenType::BitXOr, self.ch.to_string()),

            '<' => match self.peek_char() {
                '<' => {
                    self.read_char();
                    Token::new(TokenType::ShL, "<<".to_string())
                }
                '=' => {
                    self.read_char();
                    Token::new(TokenType::Lte, "<=".to_string())
                }
                _ => Token::new(TokenType::Lt, self.ch.to_string()),
            },
            '>' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenType::Gte, ">=".to_string())
                }
                '>' => {
                    self.read_char();
                    if self.peek_char() == '>' {
                        self.read_char();
                        Token::new(TokenType::SaR, ">>>".to_string())
                    } else {
                        Token::new(TokenType::ShR, ">>".to_string())
                    }
                }
                _ => Token::new(TokenType::Gt, self.ch.to_string()),
            },
            '~' => Token::new(TokenType::BitNot, self.ch.to_string()),
            ';' => Token::new(TokenType::SemiColon, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '(' => Token::new(TokenType::LParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RParen, self.ch.to_string()),
            '{' => Token::new(TokenType::LBrace, self.ch.to_string()),
            '}' => Token::new(TokenType::RBrace, self.ch.to_string()),

            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    if self.peek_char() == '=' {
                        self.read_char();
                        Token::new(TokenType::EqStrict, "===".to_string())
                    } else {
                        Token::new(TokenType::Eq, "==".to_string())
                    }
                } else {
                    Token::new(TokenType::Assign, self.ch.to_string())
                }
            }

            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    if self.peek_char() == '=' {
                        self.read_char();
                        Token::new(TokenType::NotEqStrict, "!==".to_string())
                    } else {
                        Token::new(TokenType::NotEq, "!=".to_string())
                    }
                } else {
                    Token::new(TokenType::Bang, self.ch.to_string())
                }
            }

            '?' => {
                if self.peek_char() == '?' {
                    self.read_char();
                    Token::new(TokenType::NullishCoalescing, "??".to_string())
                } else {
                    Token::new(TokenType::Conditional, self.ch.to_string())
                }
            }

            '"' | '\'' => Token::new(TokenType::String, self.read_string()),

            _ => {
                if Self::is_letter(self.ch) {
                    let id = self.read_identifier();
                    let token_type = lookup_indent(&id);
                    Token::new(token_type, id)
                } else if Self::is_digit(self.ch) {
                    Token::new(TokenType::Number, self.read_number())
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
        while Self::is_digit(self.ch)
            || self.ch == '.'
            || self.ch == 'e'
            || self.ch == '-'
            || self.ch == 'b'
            || self.ch == 'o'
            || self.ch == 'x'
            // for hex
            || self.ch == 'a'
            || self.ch == 'c'
            || self.ch == 'd'
            || self.ch == 'f'
        {
            self.read_char();
        }
        self.read_position -= 1;
        self.input[position..self.position].to_string()
    }

    fn read_string(&mut self) -> String {
        let quote = self.ch;
        let position = self.position + 1;
        self.read_char();

        loop {
            if self.ch == quote && self.input.chars().nth(self.position - 1).unwrap() != '\\' {
                break;
            }
            self.read_char();
        }

        self.input[position..self.position]
            .chars()
            .filter(|x| x != &'\\')
            .collect::<String>()
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
    fn test_whitespace() {
        let source = String::from(" \t\n\r=");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Assign);
    }

    #[test]
    fn test_digit() {
        let source = String::from("42;");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Number);
        assert_eq!(l.next_token().token_type, TokenType::SemiColon);
    }

    #[test]
    fn test_decimal() {
        let source = String::from("4.2;");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Number);
        assert_eq!(l.next_token().token_type, TokenType::SemiColon);
    }

    #[test]
    fn test_number_exp() {
        {
            let source = String::from("4e2;");
            let mut l = Lexer::new(source);
            assert_eq!(l.next_token().token_type, TokenType::Number);
            assert_eq!(l.next_token().token_type, TokenType::SemiColon);
        }
        {
            let source = String::from("4e-2;");
            let mut l = Lexer::new(source);
            assert_eq!(l.next_token().token_type, TokenType::Number);
            assert_eq!(l.next_token().token_type, TokenType::SemiColon);
        }
    }

    #[test]
    fn test_string() {
        {
            let source = String::from("\"hello world\";");
            let mut l = Lexer::new(source);
            let s = l.next_token();
            assert_eq!(s, Token::new(TokenType::String, "hello world".to_string()));
            assert_eq!(l.next_token().token_type, TokenType::SemiColon);
        }
        {
            let source = String::from("'hello world';");
            let mut l = Lexer::new(source);
            let s = l.next_token();
            assert_eq!(s, Token::new(TokenType::String, "hello world".to_string()));
            assert_eq!(l.next_token().token_type, TokenType::SemiColon);
        }
        {
            let source = String::from("'I\\'m Ubugeeei!';");
            let mut l = Lexer::new(source);
            let s = l.next_token();
            assert_eq!(
                s,
                Token::new(TokenType::String, "I'm Ubugeeei!".to_string())
            );
            assert_eq!(l.next_token().token_type, TokenType::SemiColon);
        }
    }

    #[test]
    fn test_symbol_token() {
        let source = String::from("=+-*/%!<>(){},;?|&^~");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Assign);
        assert_eq!(l.next_token().token_type, TokenType::Plus);
        assert_eq!(l.next_token().token_type, TokenType::Minus);
        assert_eq!(l.next_token().token_type, TokenType::Asterisk);
        assert_eq!(l.next_token().token_type, TokenType::Slash);
        assert_eq!(l.next_token().token_type, TokenType::Percent);
        assert_eq!(l.next_token().token_type, TokenType::Bang);
        assert_eq!(l.next_token().token_type, TokenType::Lt);
        assert_eq!(l.next_token().token_type, TokenType::Gt);
        assert_eq!(l.next_token().token_type, TokenType::LParen);
        assert_eq!(l.next_token().token_type, TokenType::RParen);
        assert_eq!(l.next_token().token_type, TokenType::LBrace);
        assert_eq!(l.next_token().token_type, TokenType::RBrace);
        assert_eq!(l.next_token().token_type, TokenType::Comma);
        assert_eq!(l.next_token().token_type, TokenType::SemiColon);
        assert_eq!(l.next_token().token_type, TokenType::Conditional);
        assert_eq!(l.next_token().token_type, TokenType::BitOr);
        assert_eq!(l.next_token().token_type, TokenType::BitAnd);
        assert_eq!(l.next_token().token_type, TokenType::BitXOr);
        assert_eq!(l.next_token().token_type, TokenType::BitNot);
    }

    #[test]
    fn test_combination_of_symbols() {
        let source = String::from("== != === !== <= >= ++ -- ** || && ?? << >> >>> typeof");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Eq);
        assert_eq!(l.next_token().token_type, TokenType::NotEq);
        assert_eq!(l.next_token().token_type, TokenType::EqStrict);
        assert_eq!(l.next_token().token_type, TokenType::NotEqStrict);
        assert_eq!(l.next_token().token_type, TokenType::Lte);
        assert_eq!(l.next_token().token_type, TokenType::Gte);
        assert_eq!(l.next_token().token_type, TokenType::Inc);
        assert_eq!(l.next_token().token_type, TokenType::Dec);
        assert_eq!(l.next_token().token_type, TokenType::Exp);
        assert_eq!(l.next_token().token_type, TokenType::Or);
        assert_eq!(l.next_token().token_type, TokenType::And);
        assert_eq!(l.next_token().token_type, TokenType::NullishCoalescing);
        assert_eq!(l.next_token().token_type, TokenType::ShL);
        assert_eq!(l.next_token().token_type, TokenType::ShR);
        assert_eq!(l.next_token().token_type, TokenType::SaR);
        assert_eq!(l.next_token().token_type, TokenType::Typeof);
    }

    #[test]
    fn test_keywords() {
        let source = String::from("function let const true false if else return null undefined");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::Function);
        assert_eq!(l.next_token().token_type, TokenType::Let);
        assert_eq!(l.next_token().token_type, TokenType::Const);
        assert_eq!(l.next_token().token_type, TokenType::True);
        assert_eq!(l.next_token().token_type, TokenType::False);
        assert_eq!(l.next_token().token_type, TokenType::If);
        assert_eq!(l.next_token().token_type, TokenType::Else);
        assert_eq!(l.next_token().token_type, TokenType::Return);
        assert_eq!(l.next_token().token_type, TokenType::Null);
        assert_eq!(l.next_token().token_type, TokenType::Undefined);
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
        assert_eq!(t.token_type, TokenType::Number);
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
        assert_eq!(t.token_type, TokenType::Number);
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
        assert_eq!(t.token_type, TokenType::Number);
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

    #[test]
    fn test_skip_line_comment() {
        let source = String::from(
            r#"
                let five = 5;
                // This is a comment
                let ten = 10;
                // This is a comment
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
        assert_eq!(t.token_type, TokenType::Number);
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
        assert_eq!(t.token_type, TokenType::Number);
        assert_eq!(t.literal, String::from("10"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SemiColon);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Eof);
    }

    #[test]
    fn test_skip_block_comment() {
        let source = String::from(
            r#"
                let five = 5;
                /* This is a comment */
                let ten = 10;
                /**
                 * This is a comment
                 */
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
        assert_eq!(t.token_type, TokenType::Number);
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
        assert_eq!(t.token_type, TokenType::Number);
        assert_eq!(t.literal, String::from("10"));

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::SemiColon);

        t = l.next_token();
        assert_eq!(t.token_type, TokenType::Eof);
    }
}
