use std::io::{Error, ErrorKind};

use crate::engine::{
    parse::{
        ast::{IfStatement, Precedence, Statement},
        parser::Parser,
    },
    tokenize::token::TokenType,
};

impl<'a> Parser<'a> {
    pub(in super::super::super) fn parse_if_statement(&mut self) -> Result<Statement, Error> {
        // guard
        if self.peeked_token.token_type != TokenType::LParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected token '(' but found {}", self.peeked_token.literal),
            ));
        }
        self.next_token(); // skip '('

        // parse test
        self.next_token();
        let test = self.parse_expression(Precedence::Lowest)?;

        // guard
        if self.peeked_token.token_type != TokenType::RParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token ')' but found '{}' (at parse_if_statement)",
                    self.peeked_token.literal
                ),
            ));
        }

        self.next_token();
        self.next_token(); // skip ')'

        // parse consequence
        let consequence = Box::new(self.parse_statement()?);

        // parse alternate
        let alternate = match self.peeked_token.token_type {
            TokenType::Else => {
                self.next_token();
                // skip 'else'
                self.next_token();
                Box::new(Some(self.parse_statement()?))
            }
            _ => Box::new(None),
        };

        Ok(Statement::If(IfStatement::new(
            test,
            consequence,
            alternate,
        )))
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        parse::{
            ast::{
                BinaryExpression, BlockStatement, Expression, IfStatement, LetStatement, Statement,
            },
            parser::Parser,
        },
        tokenize::lexer::Lexer,
    };

    #[test]
    fn test_parse_if_statements() {
        let case = vec![
            (
                String::from(
                    r#"
                      if (x < y) {
                          let a = 1;
                      } else {
                          let a = 2;
                      }
                  "#,
                ),
                vec![Statement::If(IfStatement::new(
                    Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Identifier(String::from("x"))),
                        String::from("<"),
                        Box::new(Expression::Identifier(String::from("y"))),
                    )),
                    Box::new(Statement::Block(BlockStatement::new(vec![Statement::Let(
                        LetStatement::new(String::from("a"), Expression::Number(1.0)),
                    )]))),
                    Box::new(Some(Statement::Block(BlockStatement::new(vec![
                        Statement::Let(LetStatement::new(
                            String::from("a"),
                            Expression::Number(2.0),
                        )),
                    ])))),
                ))],
            ),
            (
                String::from(
                    r#"
                      if (x < y) {
                          let a = 1;
                      }
                  "#,
                ),
                vec![Statement::If(IfStatement::new(
                    Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Identifier(String::from("x"))),
                        String::from("<"),
                        Box::new(Expression::Identifier(String::from("y"))),
                    )),
                    Box::new(Statement::Block(BlockStatement::new(vec![Statement::Let(
                        LetStatement::new(String::from("a"), Expression::Number(1.0)),
                    )]))),
                    Box::new(None),
                ))],
            ),
            (
                String::from(
                    r#"
                      if (x < y) let a = 1;
                  "#,
                ),
                vec![Statement::If(IfStatement::new(
                    Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Identifier(String::from("x"))),
                        String::from("<"),
                        Box::new(Expression::Identifier(String::from("y"))),
                    )),
                    Box::new(Statement::Let(LetStatement::new(
                        String::from("a"),
                        Expression::Number(1.0),
                    ))),
                    Box::new(None),
                ))],
            ),
        ];

        for (source, expected) in case {
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements, expected);
        }
    }
}
