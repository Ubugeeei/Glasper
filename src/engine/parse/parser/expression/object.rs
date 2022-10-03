use std::io::{Error, ErrorKind};

use crate::engine::{
    parse::{
        ast::{
            ArrayExpression, Expression, MemberExpression, ObjectExpression, ObjectProperty,
            Precedence,
        },
        parser::Parser,
    },
    tokenize::token::TokenType,
};

impl<'a> Parser<'a> {
    pub(in super::super) fn parse_object(&mut self) -> Result<Expression, Error> {
        self.next_token(); // skip '{'
        let mut properties = Vec::new();
        while self.cur_token.token_type != TokenType::RBrace {
            let prop = self.parse_object_property()?;
            properties.push(prop);
            if self.cur_token.token_type == TokenType::Comma {
                self.next_token();
            }
        }

        Ok(Expression::Object(ObjectExpression::new(properties)))
    }

    pub(in super::super) fn parse_object_property(&mut self) -> Result<ObjectProperty, Error> {
        if self.cur_token.token_type != TokenType::Ident {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Uncaught SyntaxError: Unexpected token '{}'",
                    self.cur_token.literal
                ),
            ));
        }

        let key = self.cur_token.literal.to_string();

        if self.peeked_token.token_type != TokenType::Colon {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "unexpected token \"{}\" in parse_object_property.",
                    self.peeked_token.literal
                ),
            ));
        }

        self.next_token();

        // skip ':'
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;
        self.next_token();

        if self.cur_token.token_type == TokenType::Comma {
            self.next_token();
        }
        Ok(ObjectProperty::new(key, value))
    }

    pub(in super::super) fn parse_array(&mut self) -> Result<Expression, Error> {
        self.next_token(); // skip '['
        let mut elements = Vec::new();
        while self.cur_token.token_type != TokenType::RBracket {
            let element = self.parse_expression(Precedence::Lowest)?;
            elements.push(element);
            self.next_token();
            if self.cur_token.token_type == TokenType::Comma {
                self.next_token();
            }
        }
        Ok(Expression::Array(ArrayExpression::new(elements)))
    }

    pub(in super::super) fn parse_member_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, Error> {
        self.next_token(); // skip '.'

        // TODO: dynamic member expression
        let ident = self.cur_token.literal.to_string();
        let expr = Expression::Member(Box::new(MemberExpression::new(
            Box::new(left),
            Box::new(Expression::String(ident)),
        )));
        Ok(expr)
    }

    pub(in super::super) fn parse_dynamic_member_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, Error> {
        self.next_token(); // skip '['
        let right = self.parse_expression(Precedence::Lowest)?;
        self.next_token();
        let expr = Expression::Member(Box::new(MemberExpression::new(
            Box::new(left),
            Box::new(right),
        )));
        Ok(expr)
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        parse::{
            ast::{
                ArrayExpression, BinaryExpression, ConstStatement, Expression, MemberExpression,
                ObjectExpression, ObjectProperty, Statement,
            },
            parser::Parser,
        },
        tokenize::lexer::Lexer,
    };

    #[test]
    fn test_parse_object_expression() {
        let case = vec![
            (
                r#"
                const ob = {
                    prop: {
                        value: 1,
                    },
                };
            "#
                .to_string(),
                Statement::Const(ConstStatement::new(
                    String::from("ob"),
                    Expression::Object(ObjectExpression::new(vec![ObjectProperty::new(
                        String::from("prop"),
                        Expression::Object(ObjectExpression::new(vec![ObjectProperty::new(
                            String::from("value"),
                            Expression::Number(1.0),
                        )])),
                    )])),
                )),
            ),
            (
                r#"ob.prop;"#.to_string(),
                Statement::Expression(Expression::Member(Box::new(MemberExpression::new(
                    Box::new(Expression::Identifier(String::from("ob"))),
                    Box::new(Expression::String(String::from("prop"))),
                )))),
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
    fn test_parse_dynamic_member_expression() {
        let case = vec![
            (
                r#"ob["prop"];"#.to_string(),
                Statement::Expression(Expression::Member(Box::new(MemberExpression::new(
                    Box::new(Expression::Identifier(String::from("ob"))),
                    Box::new(Expression::String(String::from("prop"))),
                )))),
            ),
            (
                r#"ob[1 + 2];"#.to_string(),
                Statement::Expression(Expression::Member(Box::new(MemberExpression::new(
                    Box::new(Expression::Identifier(String::from("ob"))),
                    Box::new(Expression::Binary(BinaryExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("+"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                )))),
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
    fn test_parse_array_expression() {
        let case = vec![
            (
                r#"
                const arr = [1, 2, 3];
            "#
                .to_string(),
                Statement::Const(ConstStatement::new(
                    String::from("arr"),
                    Expression::Array(ArrayExpression::new(vec![
                        Expression::Number(1.0),
                        Expression::Number(2.0),
                        Expression::Number(3.0),
                    ])),
                )),
            ),
            // (
            //     r#"arr[1];"#.to_string(),
            //     Statement::Expression(Expression::Index(Box::new(IndexExpression::new(
            //         Box::new(Expression::Identifier(String::from("arr"))),
            //         Box::new(Expression::Number(1.0)),
            //     )))),
            // ),
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
