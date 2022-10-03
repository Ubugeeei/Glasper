pub mod expression;
pub mod statement;

use std::io::{Error, ErrorKind};

use crate::engine::tokenize::token::TokenType;

use super::{
    super::{lexer::Lexer, token::Token},
    ast::{
        ArrayExpression, BinaryExpression, CallExpression, Expression, FunctionExpression,
        FunctionParameter, MemberExpression, ObjectExpression, ObjectProperty, Precedence, Program,
        Statement, UnaryExpression, UpdateExpression,
    },
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
                Ok(stmt) => {
                    program.statements.push(stmt);
                    self.next_token();
                }
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
            TokenType::Const => self.parse_const_statement(),
            TokenType::If => self.parse_if_statement(),
            TokenType::Switch => self.parse_switch_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::LBrace => self.parse_block_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_break_statement(&mut self) -> Result<Statement, Error> {
        self.next_token();
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token();
        }
        Ok(Statement::Break)
    }

    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        self.next_token();

        if self.cur_token.token_type == TokenType::SemiColon {
            return Ok(Statement::Return(Expression::Undefined));
        }

        let value: Expression = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token()
        }
        Ok(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token.token_type == TokenType::SemiColon {
            self.next_token()
        }

        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, Error> {
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

    fn parse_number(&mut self) -> Result<f64, Error> {
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

    fn parse_string(&mut self) -> Result<String, Error> {
        Ok(self.cur_token.literal.to_string())
    }

    fn parse_boolean(&mut self) -> Result<bool, Error> {
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

    fn parse_object(&mut self) -> Result<Expression, Error> {
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

    fn parse_object_property(&mut self) -> Result<ObjectProperty, Error> {
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

    fn parse_array(&mut self) -> Result<Expression, Error> {
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

    fn parse_unary_expression(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        self.next_token();

        let right = self.parse_expression(Precedence::Unary)?;
        let expr = Expression::Unary(UnaryExpression::new(token.literal, Box::new(right)));
        Ok(expr)
    }

    fn parse_update_expression(&mut self) -> Result<Expression, Error> {
        let ident = self.cur_token.literal.to_string();
        self.next_token();
        let update_token = self.cur_token.clone();
        let expr = Expression::Update(UpdateExpression::new(update_token.literal, ident));
        Ok(expr)
    }

    fn parse_binary_expression(&mut self, left: Expression) -> Result<Expression, Error> {
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

    fn parse_member_expression(&mut self, left: Expression) -> Result<Expression, Error> {
        self.next_token(); // skip '.'

        // TODO: dynamic member expression
        let ident = self.cur_token.literal.to_string();
        let expr = Expression::Member(Box::new(MemberExpression::new(
            Box::new(left),
            Box::new(Expression::String(ident)),
        )));
        Ok(expr)
    }

    fn parse_dynamic_member_expression(&mut self, left: Expression) -> Result<Expression, Error> {
        self.next_token(); // skip '['
        let right = self.parse_expression(Precedence::Lowest)?;
        self.next_token();
        let expr = Expression::Member(Box::new(MemberExpression::new(
            Box::new(left),
            Box::new(right),
        )));
        Ok(expr)
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

    fn parse_function_expression(&mut self) -> Result<Expression, Error> {
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

    fn parse_function_parameters(&mut self) -> Result<Vec<FunctionParameter>, Error> {
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

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, Error> {
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

    fn current_precedence(&self) -> Precedence {
        self.cur_token.clone().get_precedence()
    }
    fn peek_precedence(&self) -> Precedence {
        self.peeked_token.clone().get_precedence()
    }

    fn is_reserved_keyword(&self, ident: &str) -> bool {
        matches!(
            ident,
            "function"
                | "let"
                | "const"
                | "true"
                | "false"
                | "if"
                | "else"
                | "return"
                | "null"
                | "undefined"
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::engine::parse::ast::{BlockStatement, ConstStatement, LetStatement};

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
