use std::io::Error;

use crate::engine::{
    eval::{
        environment::Environment,
        object::{GBoolean, GNull, GNumber, GUndefined, Object},
    },
    parse::ast::{
        BlockStatement, ConstStatement, Expression, IfStatement, LetStatement, Program, Statement,
    },
};

use super::environment::{Variable, VariableKind};

pub struct Evaluator<'a> {
    env: &'a mut Environment,
}
impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Evaluator { env }
    }

    pub fn eval(&mut self, program: &Program) -> Result<Object, Error> {
        let mut result = Object::Undefined(GUndefined);
        for statement in &program.statements {
            result = self.eval_statement(statement)?;
        }
        Ok(result)
    }

    fn eval_statement(&mut self, statement: &Statement) -> Result<Object, Error> {
        match statement {
            Statement::Expression(expr) => self.eval_expression(expr),
            Statement::Let(stmt) => self.eval_let_statement(stmt),
            Statement::Const(stmt) => self.eval_const_statement(stmt),
            Statement::If(stmt) => self.eval_if_statement(stmt),
            Statement::Block(stmt) => self.eval_block_statement(stmt),
            _ => Ok(Object::Undefined(GUndefined)),
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Result<Object, Error> {
        match expr {
            Expression::Number(i) => Ok(Object::Number(GNumber { value: *i })),
            Expression::Boolean(b) => Ok(Object::Boolean(GBoolean { value: *b })),
            Expression::Null => Ok(Object::Null(GNull)),
            Expression::Undefined => Ok(Object::Undefined(GUndefined)),

            Expression::Identifier(name) => self.eval_identifier(name),

            Expression::Prefix(expr) => self.eval_prefix_expression(expr),
            Expression::Infix(expr) => {
                if expr.operator == "=" {
                    self.eval_assign_expression(&expr.left, &expr.right)
                } else {
                    let left = self.eval_expression(&expr.left)?;
                    let right = self.eval_expression(&expr.right)?;
                    self.eval_infix_expression(expr.operator.clone(), left, right)
                }
            }

            _ => Ok(Object::Undefined(GUndefined)),
        }
    }

    fn eval_prefix_expression(
        &mut self,
        expr: &crate::engine::parse::ast::PrefixExpression,
    ) -> Result<Object, Error> {
        let right = self.eval_expression(&expr.right)?;
        match expr.operator.as_str() {
            "!" => self.eval_bang_operator_expression(right),
            "-" => self.eval_minus_prefix_operator_expression(right),
            o => Err(Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unexpected prefix operator '{}'. at eval_prefix_expression",
                    o
                ),
            )),
        }
    }

    fn eval_bang_operator_expression(&self, right: Object) -> Result<Object, Error> {
        match right {
            Object::Boolean(GBoolean { value }) => Ok(Object::Boolean(GBoolean { value: !value })),
            Object::Null(_) => Ok(Object::Boolean(GBoolean { value: true })),
            Object::Undefined(_) => Ok(Object::Boolean(GBoolean { value: true })),
            Object::Number(GNumber { value }) => {
                if value == 0.0 {
                    Ok(Object::Boolean(GBoolean { value: true }))
                } else {
                    Ok(Object::Boolean(GBoolean { value: false }))
                }
            }
        }
    }

    fn eval_minus_prefix_operator_expression(&self, right: Object) -> Result<Object, Error> {
        if let Object::Number(GNumber { value }) = right {
            Ok(Object::Number(GNumber { value: -value }))
        } else {
            Err(Error::new(
                std::io::ErrorKind::Other,
                "Unexpected prefix operator. at eval_minus_prefix_operator_expression",
            ))
        }
    }

    fn eval_infix_expression(
        &self,
        operator: String,
        left: Object,
        right: Object,
    ) -> Result<Object, Error> {
        match (left, right) {
            (Object::Number(GNumber { value: l }), Object::Number(GNumber { value: r })) => {
                match operator.as_str() {
                    "+" => Ok(Object::Number(GNumber::new(l + r))),
                    "-" => Ok(Object::Number(GNumber::new(l - r))),
                    "*" => Ok(Object::Number(GNumber::new(l * r))),
                    "/" => Ok(Object::Number(GNumber::new(l / r))),
                    "%" => Ok(Object::Number(GNumber::new(l % r))),
                    "|" => Ok(Object::Number(GNumber::new((l as i64 | r as i64) as f64))),
                    "&" => Ok(Object::Number(GNumber::new((l as i64 & r as i64) as f64))),
                    "^" => Ok(Object::Number(GNumber::new((l as i64 ^ r as i64) as f64))),
                    "<" => Ok(Object::Boolean(GBoolean::new(l < r))),
                    ">" => Ok(Object::Boolean(GBoolean::new(l > r))),
                    "==" => Ok(Object::Boolean(GBoolean::new(l == r))),
                    "!=" => Ok(Object::Boolean(GBoolean::new(l != r))),
                    "**" => Ok(Object::Number(GNumber::new(l.powf(r)))),
                    "??" => Ok(Object::Number(GNumber::new(l))),
                    "||" => {
                        if l == 0.0 {
                            Ok(Object::Number(GNumber::new(r)))
                        } else {
                            Ok(Object::Number(GNumber::new(l)))
                        }
                    }
                    "&&" => {
                        if l == 0.0 {
                            Ok(Object::Number(GNumber::new(l)))
                        } else {
                            Ok(Object::Number(GNumber::new(r)))
                        }
                    }
                    "<<" => Ok(Object::Number(GNumber::new(
                        ((l as i64) << r as i64) as f64,
                    ))),
                    ">>" => Ok(Object::Number(GNumber::new((l as i64 >> r as i64) as f64))),
                    // TODO: implement
                    // ">>>" => ,
                    o => Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            "Unexpected infix operator '{}'. at eval_infix_expression",
                            o
                        ),
                    )),
                }
            }
            (Object::Boolean(GBoolean { value: l }), Object::Boolean(GBoolean { value: r })) => {
                match operator.as_str() {
                    "==" => Ok(Object::Boolean(GBoolean::new(l == r))),
                    "!=" => Ok(Object::Boolean(GBoolean::new(l != r))),
                    "??" => Ok(Object::Boolean(GBoolean::new(l))),
                    "||" => {
                        if l {
                            Ok(Object::Boolean(GBoolean::new(l)))
                        } else {
                            Ok(Object::Boolean(GBoolean::new(r)))
                        }
                    }
                    "&&" => {
                        if l {
                            Ok(Object::Boolean(GBoolean::new(r)))
                        } else {
                            Ok(Object::Boolean(GBoolean::new(l)))
                        }
                    }
                    o => Err(Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            "Unexpected infix operator '{}'. at eval_infix_expression",
                            o
                        ),
                    )),
                }
            }
            (Object::Null(_), r) | (Object::Undefined(_), r) => match operator.as_str() {
                "??" => Ok(r),
                "||" => Ok(r),
                "&&" => match left {
                    Object::Null(_) => Ok(Object::Null(GNull)),
                    Object::Undefined(_) => Ok(Object::Undefined(GUndefined)),
                    _ => unreachable!(),
                },
                o => Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Unexpected infix operator '{}'. at eval_infix_expression",
                        o
                    ),
                )),
            },
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Unexpected infix operator. at eval_infix_expression",
            )),
        }
    }

    fn eval_let_statement(&mut self, stmt: &LetStatement) -> Result<Object, Error> {
        match self.env.get(stmt.name.as_str()) {
            // varidation
            Some(var) => match var.kind {
                VariableKind::Const => Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Cannot reassign to const '{}'", stmt.name),
                )),
                VariableKind::Let | VariableKind::Var => {
                    let value = self.eval_expression(&stmt.value)?;
                    let var = Variable::new(VariableKind::Let, value);
                    self.env.set(&stmt.name, var);
                    Ok(Object::Undefined(GUndefined))
                }
            },
            // initial set
            None => {
                let value = self.eval_expression(&stmt.value)?;
                let var = Variable::new(VariableKind::Let, value);
                self.env.set(&stmt.name, var);
                Ok(Object::Undefined(GUndefined))
            }
        }
    }

    fn eval_const_statement(&mut self, stmt: &ConstStatement) -> Result<Object, Error> {
        match self.env.get(stmt.name.as_str()) {
            // varidation
            Some(var) => match var.kind {
                VariableKind::Const => Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Uncaught SyntaxError: Identifier '{}' has already been declared",
                        stmt.name
                    ),
                )),
                VariableKind::Let | VariableKind::Var => {
                    let value = self.eval_expression(&stmt.value)?;
                    let var = Variable::new(VariableKind::Const, value);
                    self.env.set(&stmt.name, var);
                    Ok(Object::Undefined(GUndefined))
                }
            },
            // initial set
            None => {
                let value = self.eval_expression(&stmt.value)?;
                let var = Variable::new(VariableKind::Const, value);
                self.env.set(&stmt.name, var);
                Ok(Object::Undefined(GUndefined))
            }
        }
    }

    fn eval_identifier(&self, name: &str) -> Result<Object, Error> {
        match self.env.get(name) {
            Some(var) => Ok(var.value),
            None => Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Uncaught ReferenceError: {} is not defined", name),
            )),
        }
    }

    fn eval_assign_expression(
        &mut self,
        left: &Expression,
        right: &Expression,
    ) -> Result<Object, Error> {
        match left {
            Expression::Identifier(name) => {
                match self.env.get(name.as_str()) {
                    Some(var) => match var.kind {
                        // varidation
                        VariableKind::Const => Err(Error::new(
                            std::io::ErrorKind::Other,
                            "Uncaught TypeError: Assignment to constant variable.",
                        )),
                        // assqign
                        VariableKind::Let => {
                            let value = self.eval_expression(right)?;
                            let var = Variable::new(VariableKind::Let, value);
                            self.env.set(name, var);
                            Ok(value)
                        }
                        VariableKind::Var => {
                            let value = self.eval_expression(right)?;
                            let var = Variable::new(VariableKind::Var, value);
                            self.env.set(name, var);
                            Ok(value)
                        }
                    },
                    // no var
                    None => {
                        let value = self.eval_expression(right)?;
                        let var = Variable::new(VariableKind::Var, value);
                        self.env.set(name, var);
                        Ok(value)
                    }
                }
            }
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Uncaught SyntaxError: Invalid left-hand side in assignment",
            )),
        }
    }

    fn eval_if_statement(&mut self, statement: &IfStatement) -> Result<Object, Error> {
        let condition = self.eval_expression(&statement.condition)?;
        if self.is_truthy(condition) {
            self.eval_statement(&statement.consequence)
        } else {
            let un_boxed = statement.alternative.as_ref();
            match un_boxed {
                Some(ref alt) => self.eval_statement(alt),
                None => Ok(Object::Undefined(GUndefined)),
            }
        }
    }

    fn eval_block_statement(&mut self, block: &BlockStatement) -> Result<Object, Error> {
        let mut result = Object::Undefined(GUndefined);
        for stmt in &block.statements {
            result = self.eval_statement(stmt)?;
        }
        Ok(result)
    }

    fn is_truthy(&self, obj: Object) -> bool {
        match obj {
            Object::Boolean(b) => b.value,
            Object::Null(_) => false,
            Object::Undefined(_) => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{parse::parser::Parser, tokenize::lexer::Lexer};

    #[test]
    fn test_eval_let_statement() {
        let mut l = Lexer::new("let a = 1;".to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        let mut e = Environment::new();
        let mut ev = Evaluator::new(&mut e);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            format!("{}", ev.eval(&program).unwrap()),
            "\x1b[30mundefined\x1b[0m"
        );
    }

    #[test]
    fn test_eval_int() {
        let mut l = Lexer::new("1".to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        let mut e = Environment::new();
        let mut ev = Evaluator::new(&mut e);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            format!("{}", ev.eval(&program).unwrap()),
            "\x1b[33m1\x1b[0m"
        );
    }

    #[test]
    fn test_eval_bool() {
        {
            let mut l = Lexer::new("true".to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33mtrue\x1b[0m"
            );
        }
        {
            let mut l = Lexer::new("false".to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33mfalse\x1b[0m"
            );
        }
    }

    #[test]
    fn test_eval_prefix_expression() {
        let case = vec![
            ("!true", "\x1b[33mfalse\x1b[0m"),
            ("!false", "\x1b[33mtrue\x1b[0m"),
            ("!5", "\x1b[33mfalse\x1b[0m"),
            ("!!true", "\x1b[33mtrue\x1b[0m"),
            ("!!false", "\x1b[33mfalse\x1b[0m"),
            ("!!5", "\x1b[33mtrue\x1b[0m"),
            ("-5", "\x1b[33m-5\x1b[0m"),
            ("-10", "\x1b[33m-10\x1b[0m"),
        ];

        for (input, expected) in case {
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn test_eval_infix_expression() {
        let case = vec![
            ("1 + 1", "\x1b[33m2\x1b[0m"),
            ("1 - 1", "\x1b[33m0\x1b[0m"),
            ("1 * 1", "\x1b[33m1\x1b[0m"),
            ("1 / 1", "\x1b[33m1\x1b[0m"),
            ("5 % 5", "\x1b[33m0\x1b[0m"),
            ("5 % 2", "\x1b[33m1\x1b[0m"),
            ("2 + 3 * 4", "\x1b[33m14\x1b[0m"),
            ("2 * 3 + 4", "\x1b[33m10\x1b[0m"),
            ("2 * (3 + 4)", "\x1b[33m14\x1b[0m"),
            ("1 < 1", "\x1b[33mfalse\x1b[0m"),
            ("1 > 1", "\x1b[33mfalse\x1b[0m"),
            ("1 == 1", "\x1b[33mtrue\x1b[0m"),
            ("1 != 1", "\x1b[33mfalse\x1b[0m"),
            ("1 < 2", "\x1b[33mtrue\x1b[0m"),
            ("1 > 2", "\x1b[33mfalse\x1b[0m"),
            ("1 == 2", "\x1b[33mfalse\x1b[0m"),
            ("1 != 2", "\x1b[33mtrue\x1b[0m"),
            ("true == true", "\x1b[33mtrue\x1b[0m"),
            ("true != true", "\x1b[33mfalse\x1b[0m"),
            ("true == false", "\x1b[33mfalse\x1b[0m"),
            ("true != false", "\x1b[33mtrue\x1b[0m"),
            ("false == false", "\x1b[33mtrue\x1b[0m"),
            ("false != false", "\x1b[33mfalse\x1b[0m"),
            ("false == true", "\x1b[33mfalse\x1b[0m"),
            ("false != true", "\x1b[33mtrue\x1b[0m"),
            ("2 ** 10", "\x1b[33m1024\x1b[0m"),
            ("null ?? 1", "\x1b[33m1\x1b[0m"),
            ("undefined ?? 1", "\x1b[33m1\x1b[0m"),
            ("1 ?? 2", "\x1b[33m1\x1b[0m"),
            ("false ?? true", "\x1b[33mfalse\x1b[0m"),
            ("null || 1", "\x1b[33m1\x1b[0m"),
            ("undefined || 1", "\x1b[33m1\x1b[0m"),
            ("1 || 2", "\x1b[33m1\x1b[0m"),
            ("0 || 2", "\x1b[33m2\x1b[0m"),
            ("false || true", "\x1b[33mtrue\x1b[0m"),
            ("null && 1", "null"),
            ("undefined && 1", "\x1b[30mundefined\x1b[0m"),
            ("1 && 2", "\x1b[33m2\x1b[0m"),
            ("0 && 2", "\x1b[33m0\x1b[0m"),
            ("false || true", "\x1b[33mtrue\x1b[0m"),
            ("0b1100 | 0b0011", "\x1b[33m15\x1b[0m"),
            ("0b1100 & 0b0011", "\x1b[33m0\x1b[0m"),
            ("0b1100 ^ 0b0011", "\x1b[33m15\x1b[0m"),
            ("0b0101 << 0b0010", "\x1b[33m20\x1b[0m"),
        ];

        for (input, expected) in case {
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn test_eval_variables() {
        let case = vec![
            ("let a = 1; a;", "\x1b[33m1\x1b[0m"),
            ("let a = 1; let b = a; b;", "\x1b[33m1\x1b[0m"),
            (
                "let a = 1; let b = a; let c = a + b + 5; c;",
                "\x1b[33m7\x1b[0m",
            ),
            ("let a = 1; a = 3;", "\x1b[33m3\x1b[0m"),
        ];

        for (input, expected) in case {
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
        }
    }

    #[test]
    fn test_assign_var_varidation() {
        // reassign to let variable
        {
            let input = "let a = 1; a = 2;";
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33m2\x1b[0m"
            );
        }

        // redeclare (let)
        {
            let input = "let a = 1; let a = 2; a;";
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33m2\x1b[0m"
            );
        }

        // redeclare (let -> const)
        {
            let input = "let a = 1; const a = 2; a;";
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            assert_eq!(
                format!("{}", ev.eval(&program).unwrap()),
                "\x1b[33m2\x1b[0m"
            );
        }

        // reassign to const variable
        {
            let input = "const a = 1; a = 2;";
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            ev.eval(&program).unwrap_err();
        }

        // redeclare (const -> const)
        {
            let input = "const a = 1; const a = 2; a;";
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            ev.eval(&program).unwrap_err();
        }

        // redeclare (const -> let)
        {
            let input = "const a = 1; let a = 2; a;";
            let mut l = Lexer::new(input.to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            let mut e = Environment::new();
            let mut ev = Evaluator::new(&mut e);
            ev.eval(&program).unwrap_err();
        }
    }

    #[test]
    fn test_eval_if_statement() {
        {
            let case = vec![
                (
                    String::from(
                        r#"
                            let a = 5;
                            if (a % 2 == 0) {
                                a = 0;
                            } else {
                                a = 1;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m1\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 6;
                            if (a % 2 == 0) {
                                a = 0;
                            } else {
                                a = 1;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m0\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 3;
                            if (a % 3 == 0) {
                                a = 0;
                            } else if (a % 3 == 1) {
                                a = 1;
                            } else {
                                a = 2;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m0\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 4;
                            if (a % 3 == 0) {
                                a = 0;
                            } else if (a % 3 == 1) {
                                a = 1;
                            } else {
                                a = 2;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m1\x1b[0m",
                ),
                (
                    String::from(
                        r#"
                            let a = 5;
                            if (a % 3 == 0) {
                                a = 0;
                            } else if (a % 3 == 1) {
                                a = 1;
                            } else {
                                a = 2;
                            }
                            a;
                        "#,
                    ),
                    "\x1b[33m2\x1b[0m",
                ),
            ];

            for (input, expected) in case {
                let mut l = Lexer::new(input.to_string());
                let mut p = Parser::new(&mut l);
                let program = p.parse_program();
                let mut e = Environment::new();
                let mut ev = Evaluator::new(&mut e);
                assert_eq!(format!("{}", ev.eval(&program).unwrap()), expected);
            }
        }
    }
}