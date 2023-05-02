use std::io::{Error, ErrorKind};

use crate::engine::{
    parser::{
        ast::{
            CallExpression, Expression, FunctionExpression, FunctionParameter, Precedence,
            Statement,
        },
        parser::Parser,
    },
    lexer::token::TokenType,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_function_expression(&mut self) -> Result<Expression, Error> {
        // guard
        if self.peeked_token.token_type != TokenType::LParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token '(' but found '{}' (at parse_function_expression)",
                    self.peeked_token.literal
                ),
            ));
        }

        let params = self.parse_function_parameters()?;

        // guard
        if self.peeked_token.token_type != TokenType::LBrace {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token '{{' but found '{}'",
                    self.peeked_token.literal
                ),
            ));
        }
        self.next_token();
        let body = self.parse_block_statement()?;
        let body = match body {
            Statement::Block(b) => b,
            _ => unreachable!(),
        };
        Ok(Expression::Function(FunctionExpression::new(params, body)))
    }

    pub(super) fn parse_function_parameters(&mut self) -> Result<Vec<FunctionParameter>, Error> {
        // guard
        if self.peeked_token.token_type != TokenType::LParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token '(' but found '{}' (at parse_function_parameters)",
                    self.peeked_token.literal
                ),
            ));
        }

        self.next_token();

        self.next_token(); // skip '('
        let mut parameters: Vec<FunctionParameter> = vec![];
        while self.cur_token.token_type != TokenType::RParen {
            if self.cur_token.token_type != TokenType::Ident {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "expected token identifier but found '{}'. in parse_function_parameters",
                        self.cur_token.literal
                    ),
                ));
            }
            let name = self.cur_token.literal.to_string();
            self.next_token();

            let default = if self.cur_token.token_type == TokenType::Assign {
                self.next_token(); // skip '='
                let expr = Some(self.parse_expression(Precedence::Lowest)?);
                self.next_token();
                expr
            } else {
                None
            };
            parameters.push(FunctionParameter::new(name, default));

            if self.cur_token.token_type == TokenType::Comma {
                self.next_token(); // skip ','
            }
        }

        Ok(parameters)
    }

    pub(super) fn parse_call_expression(
        &mut self,
        function: Expression,
    ) -> Result<Expression, Error> {
        let args = self.parse_call_arguments()?;
        Ok(Expression::Call(CallExpression::new(
            Box::new(function),
            args,
        )))
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, Error> {
        let mut args: Vec<Expression> = vec![];
        self.next_token();
        while self.cur_token.token_type != TokenType::RParen {
            let arg = self.parse_expression(Precedence::Lowest)?;
            args.push(arg);

            self.next_token();
            if self.cur_token.token_type == TokenType::Comma {
                self.next_token();
            }
        }
        Ok(args)
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        parser::{
            ast::{
                BinaryExpression, BlockStatement, CallExpression, Expression, FunctionExpression,
                FunctionParameter, LetStatement, Statement,
            },
            parser::Parser,
        },
        lexer::lexer::Lexer,
    };

    #[test]
    fn test_parse_function_expression() {
        let case = vec![
            (
                String::from(
                    r#"
                    let add = function(x, y) {
                        return x + y;
                    };
        "#,
                ),
                Statement::Let(LetStatement::new(
                    String::from("add"),
                    Expression::Function(FunctionExpression::new(
                        vec![
                            FunctionParameter::new(String::from("x"), None),
                            FunctionParameter::new(String::from("y"), None),
                        ],
                        BlockStatement::new(vec![Statement::Return(Expression::Binary(
                            BinaryExpression::new(
                                Box::new(Expression::Identifier(String::from("x"))),
                                String::from("+"),
                                Box::new(Expression::Identifier(String::from("y"))),
                            ),
                        ))]),
                    )),
                )),
            ),
            (
                String::from(
                    r#"
                    let add = function(x = 0, y = 0 * 0) {
                        return x + y;
                    };
                "#,
                ),
                Statement::Let(LetStatement::new(
                    String::from("add"),
                    Expression::Function(FunctionExpression::new(
                        vec![
                            FunctionParameter::new(
                                String::from("x"),
                                Some(Expression::Number(0.0)),
                            ),
                            FunctionParameter::new(
                                String::from("y"),
                                Some(Expression::Binary(BinaryExpression::new(
                                    Box::new(Expression::Number(0.0)),
                                    String::from("*"),
                                    Box::new(Expression::Number(0.0)),
                                ))),
                            ),
                        ],
                        BlockStatement::new(vec![Statement::Return(Expression::Binary(
                            BinaryExpression::new(
                                Box::new(Expression::Identifier(String::from("x"))),
                                String::from("+"),
                                Box::new(Expression::Identifier(String::from("y"))),
                            ),
                        ))]),
                    )),
                )),
            ),
            (
                String::from("let void = function() {};"),
                Statement::Let(LetStatement::new(
                    String::from("void"),
                    Expression::Function(FunctionExpression::new(
                        vec![],
                        BlockStatement::new(vec![]),
                    )),
                )),
            ),
            (
                String::from(
                    r#"
                    let hoge = function(x = 0, y = 1 + 2 * 3 + 4) {
                        let a = 0;
                        let b = 0;
                        return x + y * a;
                    }
                ;"#,
                ),
                Statement::Let(LetStatement::new(
                    String::from("hoge"),
                    Expression::Function(FunctionExpression::new(
                        vec![
                            FunctionParameter::new(
                                String::from("x"),
                                Some(Expression::Number(0.0)),
                            ),
                            FunctionParameter::new(
                                String::from("y"),
                                Some(Expression::Binary(BinaryExpression::new(
                                    Box::new(Expression::Binary(BinaryExpression::new(
                                        Box::new(Expression::Number(1.0)),
                                        String::from("+"),
                                        Box::new(Expression::Binary(BinaryExpression::new(
                                            Box::new(Expression::Number(2.0)),
                                            String::from("*"),
                                            Box::new(Expression::Number(3.0)),
                                        ))),
                                    ))),
                                    String::from("+"),
                                    Box::new(Expression::Number(4.0)),
                                ))),
                            ),
                        ],
                        BlockStatement::new(vec![
                            Statement::Let(LetStatement::new(
                                String::from("a"),
                                Expression::Number(0.0),
                            )),
                            Statement::Let(LetStatement::new(
                                String::from("b"),
                                Expression::Number(0.0),
                            )),
                            Statement::Return(Expression::Binary(BinaryExpression::new(
                                Box::new(Expression::Identifier(String::from("x"))),
                                String::from("+"),
                                Box::new(Expression::Binary(BinaryExpression::new(
                                    Box::new(Expression::Identifier(String::from("y"))),
                                    String::from("*"),
                                    Box::new(Expression::Identifier(String::from("a"))),
                                ))),
                            ))),
                        ]),
                    )),
                )),
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
    fn test_parse_call_expression() {
        let case = vec![
            (
                String::from("add(1, 2 * 3, 4 + 5);"),
                Statement::Expression(Expression::Call(CallExpression::new(
                    Box::new(Expression::Identifier(String::from("add"))),
                    vec![
                        Expression::Number(1.0),
                        Expression::Binary(BinaryExpression::new(
                            Box::new(Expression::Number(2.0)),
                            String::from("*"),
                            Box::new(Expression::Number(3.0)),
                        )),
                        Expression::Binary(BinaryExpression::new(
                            Box::new(Expression::Number(4.0)),
                            String::from("+"),
                            Box::new(Expression::Number(5.0)),
                        )),
                    ],
                ))),
            ),
            // TODO: immediate function call
            // (
            //     String::from("(function(a, b, c){})(1, 2 * 3, 4 + 5);"),
            //     Statement::Expression(Expression::Call(CallExpression::new(
            //         Box::new(Expression::Function(FunctionExpression::new(
            //             vec![
            //                 FunctionParameter::new(String::from("a"), None),
            //                 FunctionParameter::new(String::from("b"), None),
            //                 FunctionParameter::new(String::from("c"), None),
            //             ],
            //             BlockStatement::new(vec![]),
            //         ))),
            //         vec![
            //             Expression::Number(1.0),
            //             Expression::Binary(BinaryExpression::new(
            //                 Box::new(Expression::Number(2.0)),
            //                 String::from("*"),
            //                 Box::new(Expression::Number(3.0)),
            //             )),
            //             Expression::Binary(BinaryExpression::new(
            //                 Box::new(Expression::Number(4.0)),
            //                 String::from("+"),
            //                 Box::new(Expression::Number(5.0)),
            //             )),
            //         ],
            //     ))),
            // ),
            (
                String::from("let result = (1 + add(2, 3)) * 5;"),
                Statement::Let(LetStatement::new(
                    String::from("result"),
                    Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Binary(BinaryExpression::new(
                            Box::new(Expression::Number(1.0)),
                            String::from("+"),
                            Box::new(Expression::Call(CallExpression::new(
                                Box::new(Expression::Identifier(String::from("add"))),
                                vec![Expression::Number(2.0), Expression::Number(3.0)],
                            ))),
                        ))),
                        String::from("*"),
                        Box::new(Expression::Number(5.0)),
                    )),
                )),
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
