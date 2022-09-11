// TODO: remove this
#![allow(dead_code)]

use std::io::{Error, ErrorKind};

use crate::core::tokenize::token::TokenType;

use super::{
    super::{lexer::Lexer, token::Token},
    ast::{Expression, LetStatement, Precedence, Program, Statement},
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

    fn next_token(&mut self) {
        self.cur_token = self.peeked_token.clone();
        self.peeked_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.cur_token.token_type != TokenType::Eof {
            let res = self.parse_statement();
            match res {
                Ok(stmt) => program.statements.push(stmt),
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            }
        }

        program
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
            // _ => Err(Error::new(
            //     ErrorKind::InvalidInput,
            //     format!("unexpected token {:?}", self.cur_token.token_type),
            // )),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, Error> {
        // guard
        if self.cur_token.token_type != TokenType::Let {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected token 'let' but found {}", self.cur_token.literal),
            ));
        }
        if self.peeked_token.token_type != TokenType::Ident {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected identifier but found {}",
                    self.peeked_token.literal
                ),
            ));
        }

        let token = self.cur_token.clone();

        self.next_token();
        let name = self.cur_token.literal.to_string();

        self.next_token();
        // guard
        if self.cur_token.token_type != TokenType::Assign {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected token '=' but found {}", self.cur_token.literal),
            ));
        }

        // TODO: parse expression
        while self.cur_token.token_type != TokenType::SemiColon {
            self.next_token();
        }
        let value: Expression = Expression::Integer(0);
        self.next_token();

        Ok(Statement::Let(LetStatement::new(token, name, value)))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        // guard
        if self.cur_token.token_type != TokenType::Return {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected token 'let' but found {}", self.cur_token.literal),
            ));
        }

        // TODO: parse expression
        while self.cur_token.token_type != TokenType::SemiColon {
            self.next_token();
        }
        let value = Expression::Integer(0);
        self.next_token();

        Ok(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token();
        }

        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Result<Expression, Error> {
        let expr = match self.cur_token.token_type {
            TokenType::Ident => Expression::Identifier(self.parse_identifier()?),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unexpected token {:?}", self.cur_token.token_type),
                ))
            }
        };

        // TODO: impl
        Ok(expr)
    }

    fn parse_identifier(&mut self) -> Result<String, Error> {
        Ok(self.cur_token.literal.to_string())
    }

    fn parse_operator_expression(&mut self) -> Expression {
        todo!()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::core::tokenize::token::TokenType;

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

    #[test]
    fn test_parse_let_statements() {
        // Ok
        {
            let source = String::from(
                r#"
                    let five = 5;
                    let ten = 10;
        "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 2);
            // TODO: test
        }

        // Err
        {
            let source = String::from(
                r#"
                    let = 5;
                    let 10;
        "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 0);
        }
    }

    #[test]
    fn test_parse_return_statements() {
        // Ok
        {
            let source = String::from(
                r#"
                    return 5;
                    return 10;
        "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 2);
        }
    }

    #[test]
    fn test_parse_identifier_expression() {
        {
            let source = String::from("myVar;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Identifier(String::from("myVar")))
            );
        }
    }
}
