use std::io::{Error, ErrorKind};

use crate::engine::{
    ast::{ForInit, ForStatement, Precedence, Statement},
    parsing::{lexer::token::TokenType, parser::Parser},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_for_statement(&mut self) -> Result<Statement, Error> {
        // guard
        if self.peeked_token.token_type != TokenType::LParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected token '(' but found {}", self.peeked_token.literal),
            ));
        }
        self.next_token(); // skip '('

        // parse init
        let init = match self.peeked_token.token_type {
            TokenType::SemiColon => {
                self.next_token();
                None
            }
            TokenType::Let => {
                self.next_token();
                Some(ForInit::Statement(Box::new(self.parse_let_statement()?)))
            }
            TokenType::Const => {
                self.next_token();
                Some(ForInit::Statement(Box::new(self.parse_const_statement()?)))
            }
            _ => Some(ForInit::Expression(
                self.parse_expression(Precedence::Lowest)?,
            )),
        };

        // parse test
        let test = match self.peeked_token.token_type {
            TokenType::SemiColon => {
                self.next_token();
                None
            }
            _ => {
                self.next_token();
                Some(self.parse_expression(Precedence::Lowest)?)
            }
        };

        // guard
        if self.peeked_token.token_type != TokenType::SemiColon {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token ';' but found '{}' (at parse_for_statement)",
                    self.peeked_token.literal
                ),
            ));
        }

        self.next_token(); // skip ';'

        // parse update
        let update = match self.peeked_token.token_type {
            TokenType::RParen => {
                self.next_token();
                None
            }
            _ => {
                self.next_token();
                Some(self.parse_expression(Precedence::Lowest)?)
            }
        };

        // guard
        if self.peeked_token.token_type != TokenType::RParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token ')' but found '{}' (at parse_for_statement)",
                    self.peeked_token.literal
                ),
            ));
        }

        self.next_token();
        self.next_token(); // skip ')'

        // parse body
        let body = Box::new(self.parse_statement()?);

        Ok(Statement::For(ForStatement::new(init, test, update, body)))
    }

    pub(in super::super::super) fn parse_continue_statement(&mut self) -> Result<Statement, Error> {
        self.next_token();
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token();
        }
        Ok(Statement::Continue)
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        ast::{
            BinaryExpression, BlockStatement, CallExpression, Expression, ForInit, ForStatement,
            LetStatement, MemberExpression, Statement, UpdateExpression,
        },
        parsing::{lexer::Lexer, parser::Parser},
    };

    #[test]
    fn test_parse_for_statement() {
        let case = vec![(
            r#"
                    for (let i = 0; i < 10; i++) {
                        console.log(i);
                    }
                "#
            .to_string(),
            Statement::For(ForStatement::new(
                Some(ForInit::Statement(Box::new(Statement::Let(
                    LetStatement::new(String::from("i"), Expression::Number(0.0)),
                )))),
                Some(Expression::Binary(BinaryExpression::new(
                    Box::new(Expression::Identifier(String::from("i"))),
                    String::from("<"),
                    Box::new(Expression::Number(10.0)),
                ))),
                Some(Expression::Update(UpdateExpression::new(
                    String::from("++"),
                    String::from("i"),
                ))),
                Box::new(Statement::Block(BlockStatement::new(vec![
                    Statement::Expression(Expression::Call(CallExpression::new(
                        Box::new(Expression::Member(Box::new(MemberExpression::new(
                            Box::new(Expression::Identifier(String::from("console"))),
                            Box::new(Expression::String(String::from("log"))),
                        )))),
                        vec![Expression::Identifier(String::from("i"))],
                    ))),
                ]))),
            )),
        )];

        for (source, expected) in case {
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements[0], expected);
        }
    }
}
