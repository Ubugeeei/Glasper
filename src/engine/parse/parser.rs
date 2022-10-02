use std::io::{Error, ErrorKind};

use crate::engine::tokenize::token::TokenType;

use super::{
    super::{lexer::Lexer, token::Token},
    ast::{
        ArrayExpression, BlockStatement, CallExpression, ConstStatement, Expression,
        FunctionExpression, FunctionParameter, IfStatement, InfixExpression, LetStatement,
        MemberExpression, ObjectExpression, ObjectProperty, Precedence, PrefixExpression, Program,
        Statement, SuffixExpression, SwitchCase, SwitchStatement,
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
            TokenType::Return => self.parse_return_statement(),
            TokenType::LBrace => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, Error> {
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

    fn parse_const_statement(&mut self) -> Result<Statement, Error> {
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

    fn parse_if_statement(&mut self) -> Result<Statement, Error> {
        // guard
        if self.peeked_token.token_type != TokenType::LParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected token '(' but found {}", self.peeked_token.literal),
            ));
        }
        self.next_token(); // skip '('

        // parse condition
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

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

        // parse alternative
        let alternative = match self.peeked_token.token_type {
            TokenType::Else => {
                self.next_token();
                // skip 'else'
                self.next_token();
                Box::new(Some(self.parse_statement()?))
            }
            _ => Box::new(None),
        };

        Ok(Statement::If(IfStatement::new(
            condition,
            consequence,
            alternative,
        )))
    }

    fn parse_switch_statement(&mut self) -> Result<Statement, Error> {
        // guard
        if self.peeked_token.token_type != TokenType::LParen {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("expected token '(' but found {}", self.peeked_token.literal),
            ));
        }
        self.next_token(); // skip '('

        // parse condition
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

    fn parse_switch_case_statement(&mut self) -> Result<SwitchCase, Error> {
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

    fn parse_block_statement(&mut self) -> Result<Statement, Error> {
        // guard
        if self.cur_token.token_type != TokenType::LBrace {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "expected token '{{' but found {}",
                    self.peeked_token.literal
                ),
            ));
        }

        self.next_token(); // skip '{'

        let mut statements = vec![];
        while self.cur_token.token_type != TokenType::RBrace
            && self.cur_token.token_type != TokenType::Eof
        {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        Ok(Statement::Block(BlockStatement::new(statements)))
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

            // object
            TokenType::LBrace => self.parse_object()?,

            // array
            TokenType::LBracket => self.parse_array()?,

            TokenType::Ident => match self.peeked_token.token_type {
                TokenType::Inc | TokenType::Dec => self.parse_suffix_expression()?,
                _ => Expression::Identifier(self.parse_identifier()?),
            },

            // prefix_expression
            TokenType::Bang => self.parse_prefix_expression()?,
            TokenType::Minus => self.parse_prefix_expression()?,
            TokenType::BitNot => self.parse_prefix_expression()?,
            TokenType::Typeof => self.parse_prefix_expression()?,

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
                    self.parse_infix_expression(expr)?
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
                        let unknown_prefix = &self.cur_token.literal[..2].to_string();
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("unexpected token '{}' in parse_expression.", unknown_prefix),
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

    fn parse_prefix_expression(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        self.next_token();

        let right = self.parse_expression(Precedence::Prefix)?;
        let expr = Expression::Prefix(PrefixExpression::new(token.literal, Box::new(right)));
        Ok(expr)
    }

    fn parse_suffix_expression(&mut self) -> Result<Expression, Error> {
        let ident = self.cur_token.literal.to_string();
        self.next_token();
        let suffix_token = self.cur_token.clone();
        let expr = Expression::Suffix(SuffixExpression::new(suffix_token.literal, ident));
        Ok(expr)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        let expr = Expression::Infix(InfixExpression::new(
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
                    Expression::Infix(InfixExpression::new(
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
                    Expression::Infix(InfixExpression::new(
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
                    Expression::Infix(InfixExpression::new(
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
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Boolean(false)),
                        String::from("!="),
                        Box::new(Expression::Boolean(true)),
                    ))),
                ),
                (
                    String::from("!false;"),
                    Statement::Expression(Expression::Prefix(PrefixExpression::new(
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
                Statement::Expression(Expression::Prefix(PrefixExpression::new(
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
                Statement::Expression(Expression::Prefix(PrefixExpression::new(
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
                Statement::Expression(Expression::Prefix(PrefixExpression::new(
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
                Statement::Expression(Expression::Prefix(PrefixExpression::new(
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
                Statement::Expression(Expression::Suffix(SuffixExpression::new(
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
                Statement::Expression(Expression::Suffix(SuffixExpression::new(
                    String::from("--"),
                    String::from("a"),
                )))
            );
        }
    }

    #[test]
    fn test_parse_infix_ops_expression() {
        {
            let test_case = vec![
                (
                    String::from("1 + 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("+"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 - 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("-"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 * 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("*"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 ** 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("**"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 / 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("/"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 % 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("%"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 < 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("<"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 > 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from(">"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 <= 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("<="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 >= 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from(">="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 == 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("=="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 != 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("!="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 === 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("==="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("1 !== 2;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("!=="),
                        Box::new(Expression::Number(2.0)),
                    ))),
                ),
                (
                    String::from("null ?? 1;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Null),
                        String::from("??"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 | 1;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("|"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 || 1;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("||"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 & 1;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("&"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 && 1;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("&&"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 >> 1;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from(">>"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 >>> 1;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from(">>>"),
                        Box::new(Expression::Number(1.0)),
                    ))),
                ),
                (
                    String::from("1 << 1;"),
                    Statement::Expression(Expression::Infix(InfixExpression::new(
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
                Statement::Expression(Expression::Infix(InfixExpression::new(
                    Box::new(Expression::Number(1.0)),
                    String::from("+"),
                    Box::new(Expression::Infix(InfixExpression::new(
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
                Statement::Expression(Expression::Infix(InfixExpression::new(
                    Box::new(Expression::Infix(InfixExpression::new(
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
                Statement::Expression(Expression::Infix(InfixExpression::new(
                    Box::new(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Infix(InfixExpression::new(
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
                Statement::Expression(Expression::Infix(InfixExpression::new(
                    Box::new(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Infix(InfixExpression::new(
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
                Statement::Expression(Expression::Infix(InfixExpression::new(
                    Box::new(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("+"),
                        Box::new(Expression::Infix(InfixExpression::new(
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
                Statement::Expression(Expression::Infix(InfixExpression::new(
                    Box::new(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(1.0)),
                        String::from("+"),
                        Box::new(Expression::Number(2.0)),
                    ))),
                    String::from("+"),
                    Box::new(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Number(3.0)),
                        String::from("+"),
                        Box::new(Expression::Number(4.0)),
                    ))),
                ))),
            ),
            (
                String::from("0 + ((1 + 2) + (3 + 4));"),
                Statement::Expression(Expression::Infix(InfixExpression::new(
                    Box::new(Expression::Number(0.0)),
                    String::from("+"),
                    Box::new(Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Infix(InfixExpression::new(
                            Box::new(Expression::Number(1.0)),
                            String::from("+"),
                            Box::new(Expression::Number(2.0)),
                        ))),
                        String::from("+"),
                        Box::new(Expression::Infix(InfixExpression::new(
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
                        BlockStatement::new(vec![Statement::Return(Expression::Infix(
                            InfixExpression::new(
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
                                Some(Expression::Infix(InfixExpression::new(
                                    Box::new(Expression::Number(0.0)),
                                    String::from("*"),
                                    Box::new(Expression::Number(0.0)),
                                ))),
                            ),
                        ],
                        BlockStatement::new(vec![Statement::Return(Expression::Infix(
                            InfixExpression::new(
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
                                Some(Expression::Infix(InfixExpression::new(
                                    Box::new(Expression::Infix(InfixExpression::new(
                                        Box::new(Expression::Number(1.0)),
                                        String::from("+"),
                                        Box::new(Expression::Infix(InfixExpression::new(
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
                            Statement::Return(Expression::Infix(InfixExpression::new(
                                Box::new(Expression::Identifier(String::from("x"))),
                                String::from("+"),
                                Box::new(Expression::Infix(InfixExpression::new(
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
                        Expression::Infix(InfixExpression::new(
                            Box::new(Expression::Number(2.0)),
                            String::from("*"),
                            Box::new(Expression::Number(3.0)),
                        )),
                        Expression::Infix(InfixExpression::new(
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
            //             Expression::Infix(InfixExpression::new(
            //                 Box::new(Expression::Number(2.0)),
            //                 String::from("*"),
            //                 Box::new(Expression::Number(3.0)),
            //             )),
            //             Expression::Infix(InfixExpression::new(
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
                    Expression::Infix(InfixExpression::new(
                        Box::new(Expression::Infix(InfixExpression::new(
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
                    Box::new(Expression::Infix(InfixExpression::new(
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
    fn test_parse_switch_statement() {
        let case = vec![(
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
                            SwitchCase::new(None, vec![Statement::Return(Expression::Number(3.0))]),
                        ],
                    ))]),
                )),
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
