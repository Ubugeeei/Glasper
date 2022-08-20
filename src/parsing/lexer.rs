use super::{Token, TokenType};

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
        let tok = match self.ch {
            '=' => Token::new(TokenType::ASSIGN, self.ch.to_string()),
            '+' => Token::new(TokenType::PLUS, self.ch.to_string()),
            ';' => Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            ',' => Token::new(TokenType::COMMA, self.ch.to_string()),
            '(' => Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => Token::new(TokenType::RPAREN, self.ch.to_string()),
            '{' => Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => Token::new(TokenType::RBRACE, self.ch.to_string()),
            '0' => Token::new(TokenType::EOF, self.ch.to_string()),
            _ => Token::new(TokenType::ILLEGAL, self.ch.to_string()),
        };

        self.read_char();

        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_next_token() {
        let source = String::from("=+(){},;");
        let mut l = Lexer::new(source);
        assert_eq!(l.next_token().token_type, TokenType::ASSIGN);
        assert_eq!(l.next_token().token_type, TokenType::PLUS);
        assert_eq!(l.next_token().token_type, TokenType::LPAREN);
        assert_eq!(l.next_token().token_type, TokenType::RPAREN);
        assert_eq!(l.next_token().token_type, TokenType::LBRACE);
        assert_eq!(l.next_token().token_type, TokenType::RBRACE);
        assert_eq!(l.next_token().token_type, TokenType::COMMA);
        assert_eq!(l.next_token().token_type, TokenType::SEMICOLON);
    }
}
