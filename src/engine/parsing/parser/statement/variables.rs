use std::io::{Error, ErrorKind};

use crate::engine::{
    ast::{ConstStatement, Expression, LetStatement, Precedence, Statement},
    parsing::{lexer::token::TokenType, parser::Parser},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_let_statement(&mut self) -> Result<Statement, Error> {
        self.next_token();

        // guard
        if self.is_reserved_keyword(&self.cur_token.literal) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("'{}' is reserved keyword", &self.cur_token.literal),
            ));
        }
        if self.cur_token.token_type != TokenType::Ident {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected identifier but found {}",
                    self.peeked_token.literal
                ),
            ));
        }
        let name = self.cur_token.literal.clone();

        // declare without initial value
        if self.peeked_token.token_type != TokenType::Assign {
            self.next_token();
            if self.cur_token.token_type == TokenType::SemiColon
                || self.cur_token.token_type == TokenType::Eof
            {
                return Ok(Statement::Let(LetStatement::new(
                    name,
                    Expression::Undefined,
                )));
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "Uncaught SyntaxError: Unexpected token '{}'",
                        self.cur_token.literal
                    ),
                ));
            }
        }

        // skip assign
        self.next_token();

        self.next_token();
        let value: Expression = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token()
        }
        Ok(Statement::Let(LetStatement::new(name, value)))
    }

    pub(in super::super::super) fn parse_const_statement(&mut self) -> Result<Statement, Error> {
        self.next_token();

        // guard
        if self.is_reserved_keyword(&self.cur_token.literal) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("'{}' is reserved keyword", &self.cur_token.literal),
            ));
        }
        if self.cur_token.token_type != TokenType::Ident {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected identifier but found {}",
                    self.peeked_token.literal
                ),
            ));
        }
        let name = self.cur_token.literal.clone();

        if self.peeked_token.token_type != TokenType::Assign {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected token '=' but found {}", self.cur_token.literal),
            ));
        }

        // skip assign
        self.next_token();

        self.next_token();
        let value: Expression = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token()
        }
        Ok(Statement::Const(ConstStatement::new(name, value)))
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        ast::{ConstStatement, Expression, LetStatement, Statement},
        parsing::{lexer::Lexer, parser::Parser},
    };

    #[test]
    fn test_parse_let_statements() {
        // Ok
        {
            let source = String::from(
                r#"
                  let five = 5;
                  let ten = 10;
                  let a;
                  let b = true;
                  let c = false;
                  let d = "hello world";
              "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(
                program.statements,
                vec![
                    Statement::Let(LetStatement::new(
                        String::from("five"),
                        Expression::Number(5.0)
                    )),
                    Statement::Let(LetStatement::new(
                        String::from("ten"),
                        Expression::Number(10.0)
                    )),
                    Statement::Let(LetStatement::new(String::from("a"), Expression::Undefined)),
                    Statement::Let(LetStatement::new(
                        String::from("b"),
                        Expression::Boolean(true)
                    )),
                    Statement::Let(LetStatement::new(
                        String::from("c"),
                        Expression::Boolean(false)
                    )),
                    Statement::Let(LetStatement::new(
                        String::from("d"),
                        Expression::String(String::from("hello world"))
                    )),
                ]
            );
        }

        // Err
        {
            let source = String::from(
                r#"
                  let = 5;
              "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 0);
        }
        {
            let source = String::from(
                r#"
                  let = ;
              "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 0);
        }
        {
            let source = String::from(
                r#"
                  let a a;
              "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 0);
        }
    }

    #[test]
    fn test_parse_const_statements() {
        // Ok
        {
            let source = String::from(
                r#"
                  const five = 5;
                  const ten = 10;
      "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 2);
            assert_eq!(
                program.statements,
                vec![
                    Statement::Const(ConstStatement::new(
                        String::from("five"),
                        Expression::Number(5.0)
                    )),
                    Statement::Const(ConstStatement::new(
                        String::from("ten"),
                        Expression::Number(10.0)
                    ))
                ]
            );
        }

        // Err
        {
            let source = String::from(
                r#"
                  const = 5;
                  const 10;
              "#,
            );
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 0);
        }
    }

    #[test]
    fn test_reserved_keywords_error() {
        let case = vec![
            "let function = 5;",
            "let true = 5;",
            "let false = 5;",
            "let if = 5;",
            "let else = 5;",
            "let return = 5;",
            "let null = 5;",
            "let let = 5;",
            "let const = 5;",
            "let undefined = 5;",
        ];
        for source in case {
            let mut l = Lexer::new(source.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 0);
        }
    }
}
