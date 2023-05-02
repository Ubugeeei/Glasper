pub mod function;
pub mod object;
pub mod operator;
pub mod primitive;

use std::io::{Error, ErrorKind};

use crate::engine::{
    ast::{Expression, Precedence},
    parsing::{lexer::token::TokenType, parser::Parser},
};

impl<'a> Parser<'a> {
    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, Error> {
        let mut expr = match self.cur_token.token_type {
            TokenType::True | TokenType::False => Expression::Boolean(self.parse_boolean()?),
            TokenType::Number => Expression::Number(self.parse_number()?),
            TokenType::String => Expression::String(self.parse_string()?),
            TokenType::Null => Expression::Null,
            TokenType::Undefined => Expression::Undefined,
            TokenType::NaN => Expression::NaN,
            TokenType::This => Expression::This,

            // object
            TokenType::LBrace => self.parse_object()?,

            // array
            TokenType::LBracket => self.parse_array()?,

            TokenType::Ident => match self.peeked_token.token_type {
                TokenType::Inc | TokenType::Dec => self.parse_update_expression()?,
                _ => Expression::Identifier(self.parse_identifier()?),
            },

            // unary_expression
            TokenType::Bang => self.parse_unary_expression()?,
            TokenType::Minus => self.parse_unary_expression()?,
            TokenType::BitNot => self.parse_unary_expression()?,
            TokenType::Typeof => self.parse_unary_expression()?,

            // grouped
            TokenType::LParen => self.parse_grouped_expression()?,

            TokenType::Function => self.parse_function_expression()?,

            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "unexpected token \"{}\" (at parse_expression)",
                        self.cur_token.literal
                    ),
                ))
            }
        };

        while self.peeked_token.token_type != TokenType::SemiColon
            && precedence < self.peek_precedence()
        {
            expr = match self.peeked_token.token_type {
                TokenType::LParen => {
                    if self.cur_token.token_type == TokenType::Ident
                        || self.cur_token.token_type == TokenType::RParen
                    {
                        self.next_token();
                        self.parse_call_expression(expr)?
                    } else {
                        self.parse_grouped_expression()?
                    }
                }
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Asterisk
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::Exp
                | TokenType::And
                | TokenType::Or
                | TokenType::NullishCoalescing
                | TokenType::BitAnd
                | TokenType::BitOr
                | TokenType::BitXOr
                | TokenType::Lt
                | TokenType::Gt
                | TokenType::Lte
                | TokenType::Gte
                | TokenType::Eq
                | TokenType::NotEq
                | TokenType::EqStrict
                | TokenType::NotEqStrict
                | TokenType::Assign
                | TokenType::ShL
                | TokenType::ShR
                | TokenType::SaR => {
                    self.next_token();
                    self.parse_binary_expression(expr)?
                }
                TokenType::Period => {
                    self.next_token();
                    self.parse_member_expression(expr)?
                }
                TokenType::LBracket => {
                    self.next_token();
                    self.parse_dynamic_member_expression(expr)?
                }
                _ => expr,
            }
        }
        // TODO: impl
        Ok(expr)
    }

    fn parse_identifier(&mut self) -> Result<String, Error> {
        Ok(self.cur_token.literal.to_string())
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, Error> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peeked_token.token_type == TokenType::RParen {
            // skip r paren
            self.next_token();
            Ok(expr)
        } else {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token ')' or ','. but found '{}' (at parse_grouped_expression)",
                    self.peeked_token.literal
                ),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        ast::{BinaryExpression, Expression, Statement},
        parsing::{lexer::Lexer, parser::Parser},
    };

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

    #[test]
    fn test_parse_grouped_expression() {
        let case = vec![
            (
                String::from("(1 + 2) + 3 + 4;"),
                Statement::Expression(Expression::Binary(BinaryExpression::new(
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Binary(BinaryExpression::new(
                            Box::new(Expression::Number(1.0)),
                            String::from("+"),
                            Box::new(Expression::Number(2.0)),
                        ))),
                        String::from("+"),
                        Box::new(Expression::Number(3.0)),
                    ))),
                    String::from("+"),
                    Box::new(Expression::Number(4.0)),
                ))),
            ),
            (
                String::from("1 + (2 + 3) + 4;"),
                Statement::Expression(Expression::Binary(BinaryExpression::new(
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("+"),
                        Box::new(Expression::Binary(BinaryExpression::new(
                            Box::new(Expression::Number(2.0)),
                            String::from("+"),
                            Box::new(Expression::Number(3.0)),
                        ))),
                    ))),
                    String::from("+"),
                    Box::new(Expression::Number(4.0)),
                ))),
            ),
            (
                String::from("1 + 2 + (3 + 4);"),
                Statement::Expression(Expression::Binary(BinaryExpression::new(
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("+"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                    String::from("+"),
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(3.0)),
                        String::from("+"),
                        Box::new(Expression::Number(4.0)),
                    ))),
                ))),
            ),
            (
                String::from("0 + ((1 + 2) + (3 + 4));"),
                Statement::Expression(Expression::Binary(BinaryExpression::new(
                    Box::new(Expression::Number(0.0)),
                    String::from("+"),
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Binary(BinaryExpression::new(
                            Box::new(Expression::Number(1.0)),
                            String::from("+"),
                            Box::new(Expression::Number(2.0)),
                        ))),
                        String::from("+"),
                        Box::new(Expression::Binary(BinaryExpression::new(
                            Box::new(Expression::Number(3.0)),
                            String::from("+"),
                            Box::new(Expression::Number(4.0)),
                        ))),
                    ))),
                ))),
            ),
        ];

        for (source, expected) in case {
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(program.statements[0], expected);
        }
    }

    #[test]
    fn parse_this() {
        let case = vec![(
            r#"
              this;
          "#
            .to_string(),
            Statement::Expression(Expression::This),
        )];

        for (source, expected) in case {
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements[0], expected);
        }
    }
}
