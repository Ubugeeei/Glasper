use std::io::Error;

use crate::engine::parse::{
    ast::{BinaryExpression, Expression, Precedence, UnaryExpression, UpdateExpression},
    parser::Parser,
};

impl<'a> Parser<'a> {
    pub(in super::super) fn parse_unary_expression(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        self.next_token();

        let right = self.parse_expression(Precedence::Unary)?;
        let expr = Expression::Unary(UnaryExpression::new(token.literal, Box::new(right)));
        Ok(expr)
    }

    pub(in super::super) fn parse_update_expression(&mut self) -> Result<Expression, Error> {
        let ident = self.cur_token.literal.to_string();
        self.next_token();
        let update_token = self.cur_token.clone();
        let expr = Expression::Update(UpdateExpression::new(update_token.literal, ident));
        Ok(expr)
    }

    pub(in super::super) fn parse_binary_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        let expr = Expression::Binary(BinaryExpression::new(
            Box::new(left),
            token.literal,
            Box::new(right),
        ));
        Ok(expr)
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        parse::{
            ast::{BinaryExpression, Expression, Statement, UnaryExpression, UpdateExpression},
            parser::Parser,
        },
        tokenize::lexer::Lexer,
    };

    #[test]
    fn test_parse_pre_ops_expressions() {
        {
            let source = String::from("-5;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Unary(UnaryExpression::new(
                    String::from("-"),
                    Box::new(Expression::Number(5.0))
                )))
            );
        }

        {
            let source = String::from("!flag;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Unary(UnaryExpression::new(
                    String::from("!"),
                    Box::new(Expression::Identifier(String::from("flag")))
                )))
            );
        }

        {
            let source = String::from("~flag;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Unary(UnaryExpression::new(
                    String::from("~"),
                    Box::new(Expression::Identifier(String::from("flag")))
                )))
            );
        }

        {
            let source = String::from("typeof flag;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Unary(UnaryExpression::new(
                    String::from("typeof"),
                    Box::new(Expression::Identifier(String::from("flag")))
                )))
            );
        }
    }

    #[test]
    fn test_parse_suf_ops_expressions() {
        {
            let source = String::from("a++;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Update(UpdateExpression::new(
                    String::from("++"),
                    String::from("a"),
                )))
            );
        }

        {
            let source = String::from("a--;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Update(UpdateExpression::new(
                    String::from("--"),
                    String::from("a"),
                )))
            );
        }
    }

    #[test]
    fn test_parse_binary_ops_expression() {
        {
            let test_case = vec![
                (
                    String::from("1 + 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("+"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 - 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("-"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 * 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("*"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 ** 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("**"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 / 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("/"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 % 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("%"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 < 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("<"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 > 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from(">"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 <= 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("<="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 >= 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from(">="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 == 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("=="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 != 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("!="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 === 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("==="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 !== 2;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("!=="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("null ?? 1;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Null),
                        String::from("??"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 | 1;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("|"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 || 1;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("||"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 & 1;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("&"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 && 1;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("&&"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 >> 1;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from(">>"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 >>> 1;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from(">>>"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 << 1;"),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("<<"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
            ];

            for (source, expected) in test_case {
                let mut l = Lexer::new(source);
                let mut p = Parser::new(&mut l);
                let program = p.parse_program();
                assert_eq!(program.statements.len(), 1);
                assert_eq!(program.statements[0], expected);
            }
        }

        {
            let source = String::from("1 + 2 * 3;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Binary(BinaryExpression::new(
                    Box::new(Expression::Number(1.0)),
                    String::from("+"),
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(2.0)),
                        String::from("*"),
                        Box::new(Expression::Number(3.0)),
                    )))
                )))
            );
        }

        {
            let source = String::from("1 * 2 + 3;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Binary(BinaryExpression::new(
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("*"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                    String::from("+"),
                    Box::new(Expression::Number(3.0)),
                )))
            );
        }

        {
            let source = String::from("a * 2 + 3 != 11;");
            let mut l = Lexer::new(source);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                program.statements[0],
                Statement::Expression(Expression::Binary(BinaryExpression::new(
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Binary(BinaryExpression::new(
                            Box::new(Expression::Identifier(String::from("a"))),
                            String::from("*"),
                            Box::new(Expression::Number(2.0)),
                        ))),
                        String::from("+"),
                        Box::new(Expression::Number(3.0)),
                    ))),
                    String::from("!="),
                    Box::new(Expression::Number(11.0)),
                ))),
            );
        }
    }
}
