use std::io::{Error, ErrorKind};

use crate::engine::{parser::parser::Parser, lexer::token::TokenType};

impl<'a> Parser<'a> {
    pub(super) fn parse_number(&mut self) -> Result<f64, Error> {
        let mut lit_iter = self.cur_token.literal.chars();
        if lit_iter.next() == Some('0') {
            if let Some(c) = lit_iter.next() {
                match c {
                    'b' => {
                        let bin = &self.cur_token.literal[2..];
                        return Ok(i64::from_str_radix(bin, 2).unwrap() as f64);
                    }
                    'x' => {
                        let hex = &self.cur_token.literal[2..];
                        return Ok(i64::from_str_radix(hex, 16).unwrap() as f64);
                    }
                    'o' => {
                        let oct = &self.cur_token.literal[2..];
                        return Ok(i64::from_str_radix(oct, 8).unwrap() as f64);
                    }
                    _ => {
                        let unknown_unary = &self.cur_token.literal[..2].to_string();
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("unexpected token '{}' in parse_expression.", unknown_unary),
                        ));
                    }
                }
            }
        }
        Ok(self.cur_token.literal.parse::<f64>().unwrap())
    }

    pub(super) fn parse_string(&mut self) -> Result<String, Error> {
        Ok(self.cur_token.literal.to_string())
    }

    pub(super) fn parse_boolean(&mut self) -> Result<bool, Error> {
        match self.cur_token.token_type {
            TokenType::True => Ok(true),
            TokenType::False => Ok(false),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "unexpected token \"{}\" in parse_boolean.",
                    self.cur_token.literal
                ),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        parser::{
            ast::{BinaryExpression, Expression, Statement, UnaryExpression},
            parser::Parser,
        },
        lexer::lexer::Lexer,
    };

    #[test]
    fn test_parse_number_expression() {
        {
            let source = String::from("5;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Number(5.0))
            );
        }
        {
            let source = String::from("4e4;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Number(40000.0))
            );
        }
        {
            let source = String::from("4e-4;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Number(0.0004))
            );
        }
        {
            let source = String::from("0xff;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Number(255.0))
            );
        }
        {
            let source = String::from("0b1111;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Number(15.0))
            );
        }
    }

    #[test]
    fn test_parse_boolean_expression() {
        {
            let case = vec![
                (
                    String::from("true;"),
                    Statement::Expression(Expression::Boolean(true)),
                ),
                (
                    String::from("false != true;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Boolean(false)),
                        String::from("!="),
                        Box::new(Expression::Boolean(true)),
                    ))),
                ),
                (
                    String::from("!false;"),
                    Statement::Expression(Expression::Unary(UnaryExpression::new(
                        String::from("!"),
                        Box::new(Expression::Boolean(false)),
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
    }

    #[test]
    fn test_parse_null_expression() {
        {
            let source = String::from("null;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Null)
            );
        }
    }

    #[test]
    fn test_parse_undefined_expression() {
        {
            let source = String::from("undefined;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Undefined)
            );
        }
    }
}
