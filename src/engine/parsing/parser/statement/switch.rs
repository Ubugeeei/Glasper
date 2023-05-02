use std::io::{Error, ErrorKind};

use crate::engine::{
    ast::{Precedence, Statement, SwitchCase, SwitchStatement},
    parsing::{lexer::token::TokenType, parser::Parser},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_switch_statement(&mut self) -> Result<Statement, Error> {
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
        let discriminant = self.parse_expression(Precedence::Lowest)?;

        // guard
        if self.peeked_token.token_type != TokenType::RParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token ')' but found '{}' (at parse_switch_statement)",
                    self.peeked_token.literal
                ),
            ));
        }

        self.next_token();
        self.next_token(); // skip ')'

        // parse cases
        let mut cases = Vec::new();
        while self.peeked_token.token_type != TokenType::RBrace {
            let case = self.parse_switch_case_statement()?;
            cases.push(case);
        }

        self.next_token(); // skip '}'

        Ok(Statement::Switch(SwitchStatement::new(discriminant, cases)))
    }

    pub(in super::super::super) fn parse_switch_case_statement(
        &mut self,
    ) -> Result<SwitchCase, Error> {
        // guard
        if self.peeked_token.token_type != TokenType::Case
            && self.peeked_token.token_type != TokenType::Default
        {
            return Err(Error::new(
              ErrorKind::InvalidInput,
              format!(
                  "expected token 'case' or 'default' but found '{}' (at parse_switch_case_statement)",
                  self.peeked_token.literal
              ),
          ));
        }
        self.next_token(); // skip 'case'

        match self.cur_token.token_type {
            TokenType::Case => {
                self.next_token(); // skip 'case'
                let test = self.parse_expression(Precedence::Lowest)?;
                // guard
                if self.peeked_token.token_type != TokenType::Colon {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "expected token ':' but found '{}' (at parse_switch_case_statement)",
                            self.peeked_token.literal
                        ),
                    ));
                }
                self.next_token();
                self.next_token(); // skip ':'

                // parse consequent
                let mut consequent = Vec::new();
                while self.peeked_token.token_type != TokenType::Case
                    && self.peeked_token.token_type != TokenType::Default
                    && self.peeked_token.token_type != TokenType::RBrace
                {
                    let statement = self.parse_statement()?;
                    consequent.push(statement);
                }

                Ok(SwitchCase::new(Some(test), consequent))
            }

            TokenType::Default => {
                // guard
                if self.peeked_token.token_type != TokenType::Colon {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "expected token ':' but found '{}' (at parse_switch_case_statement)",
                            self.peeked_token.literal
                        ),
                    ));
                }
                self.next_token();
                self.next_token(); // skip ':'

                // parse consequent
                let mut consequent = Vec::new();
                while self.peeked_token.token_type != TokenType::Case
                    && self.peeked_token.token_type != TokenType::Default
                    && self.peeked_token.token_type != TokenType::RBrace
                {
                    let statement = self.parse_statement()?;
                    consequent.push(statement);
                }

                Ok(SwitchCase::new(None, consequent))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        ast::{
            BlockStatement, ConstStatement, Expression, FunctionExpression, FunctionParameter,
            Statement, SwitchCase, SwitchStatement,
        },
        parsing::{lexer::Lexer, parser::Parser},
    };

    #[test]
    fn test_parse_switch_statement() {
        let case = vec![
            (
                r#"
                  const f = function(a) {
                      switch (a) {
                          case 1:
                              return 1;
                          case 2:
                              return 2;
                          default:
                              return 3;
                      }
                  };
              "#
                .to_string(),
                Statement::Const(ConstStatement::new(
                    String::from("f"),
                    Expression::Function(FunctionExpression::new(
                        vec![FunctionParameter::new(String::from("a"), None)],
                        BlockStatement::new(vec![Statement::Switch(SwitchStatement::new(
                            Expression::Identifier(String::from("a")),
                            vec![
                                SwitchCase::new(
                                    Some(Expression::Number(1.0)),
                                    vec![Statement::Return(Expression::Number(1.0))],
                                ),
                                SwitchCase::new(
                                    Some(Expression::Number(2.0)),
                                    vec![Statement::Return(Expression::Number(2.0))],
                                ),
                                SwitchCase::new(
                                    None,
                                    vec![Statement::Return(Expression::Number(3.0))],
                                ),
                            ],
                        ))]),
                    )),
                )),
            ),
            (
                r#"
                  const f = function(a) {
                      switch (a) {
                          case 1:
                              break;
                          case 2:
                              break;
                          default:
                              break;
                      }
                  };
              "#
                .to_string(),
                Statement::Const(ConstStatement::new(
                    String::from("f"),
                    Expression::Function(FunctionExpression::new(
                        vec![FunctionParameter::new(String::from("a"), None)],
                        BlockStatement::new(vec![Statement::Switch(SwitchStatement::new(
                            Expression::Identifier(String::from("a")),
                            vec![
                                SwitchCase::new(
                                    Some(Expression::Number(1.0)),
                                    vec![Statement::Break],
                                ),
                                SwitchCase::new(
                                    Some(Expression::Number(2.0)),
                                    vec![Statement::Break],
                                ),
                                SwitchCase::new(None, vec![Statement::Break]),
                            ],
                        ))]),
                    )),
                )),
            ),
        ];

        for (source, expected) in case {
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements[0], expected);
        }
    }
}
