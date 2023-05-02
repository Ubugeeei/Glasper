pub mod expression;
pub mod statement;

use crate::engine::{
    ast::{Precedence, Program},
    parsing::lexer::{
        token::{Token, TokenType},
        Lexer,
    },
};

pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peeked_token: Token,
}
impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer) -> Self {
        let first_token = l.next_token();
        let second_token = l.next_token();

        Parser {
            l,
            cur_token: first_token,
            peeked_token: second_token,
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.cur_token.token_type != TokenType::Eof {
            let res = self.parse_statement();
            match res {
                Ok(stmt) => {
                    program.statements.push(stmt);
                    self.next_token();
                }
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            }
        }

        program
    }

    fn current_precedence(&self) -> Precedence {
        self.cur_token.clone().get_precedence()
    }

    fn peek_precedence(&self) -> Precedence {
        self.peeked_token.clone().get_precedence()
    }

    fn is_reserved_keyword(&self, ident: &str) -> bool {
        matches!(
            ident,
            "function"
                | "let"
                | "const"
                | "true"
                | "false"
                | "if"
                | "else"
                | "return"
                | "null"
                | "undefined"
        )
    }

    fn next_token(&mut self) {
        self.cur_token = self.peeked_token.clone();
        self.peeked_token = self.l.next_token();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_new() {
        {
            let source = String::from("let five = 5;");
            let mut l = Lexer::new(source);
            let p = Parser::new(&mut l);

            assert_eq!(p.cur_token.token_type, TokenType::Let);
            assert_eq!(p.peeked_token.token_type, TokenType::Ident);
        }

        {
            let source = String::from("");
            let mut l = Lexer::new(source);
            let p = Parser::new(&mut l);

            assert_eq!(p.cur_token.token_type, TokenType::Eof);
            assert_eq!(p.peeked_token.token_type, TokenType::Eof);
        }
    }
}
