#![allow(dead_code)]

use std::io::Error;

use crate::core::{
    object::def::{GBoolean, GNull, GNumber, GUndefined, Object},
    parse::ast::{Expression, Program, Statement},
};

pub fn eval(_program: &Program) -> Result<Object, Error> {
    let mut result = Object::Undefined(GUndefined);
    for statement in &_program.statements {
        result = eval_statement(statement)?;
    }
    Ok(result)
}

fn eval_statement(statement: &Statement) -> Result<Object, Error> {
    match statement {
        Statement::Expression(expr) => eval_expression(expr),
        _ => Ok(Object::Undefined(GUndefined)),
    }
}

fn eval_expression(expr: &Expression) -> Result<Object, Error> {
    match expr {
        Expression::Number(i) => Ok(Object::Number(GNumber { value: *i })),
        Expression::Boolean(b) => Ok(Object::Boolean(GBoolean { value: *b })),
        Expression::Null => Ok(Object::Null(GNull)),
        Expression::Undefined => Ok(Object::Undefined(GUndefined)),

        Expression::Prefix(expr) => eval_prefix_expression(expr),

        _ => Ok(Object::Undefined(GUndefined)),
    }
}

fn eval_prefix_expression(
    expr: &crate::core::parse::ast::PrefixExpression,
) -> Result<Object, Error> {
    let right = eval_expression(&expr.right)?;
    match expr.operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        o => Err(Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Unexpected prefix operator '{}'. at eval_prefix_expression",
                o
            ),
        )),
    }
}

fn eval_bang_operator_expression(right: Object) -> Result<Object, Error> {
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

fn eval_minus_prefix_operator_expression(right: Object) -> Result<Object, Error> {
    if let Object::Number(GNumber { value }) = right {
        Ok(Object::Number(GNumber { value: -value }))
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            "Unexpected prefix operator. at eval_minus_prefix_operator_expression",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{parse::parser::Parser, tokenize::lexer::Lexer};

    #[test]
    fn test_eval_let_statement() {
        let mut l = Lexer::new("let a = 1;".to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            format!("{}", eval(&program).unwrap()),
            "\x1b[30mundefined\x1b[0m"
        );
    }

    #[test]
    fn test_eval_int() {
        let mut l = Lexer::new("1".to_string());
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert_eq!(program.statements.len(), 1);
        assert_eq!(format!("{}", eval(&program).unwrap()), "\x1b[33m1\x1b[0m");
    }

    #[test]
    fn test_eval_bool() {
        {
            let mut l = Lexer::new("true".to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                format!("{}", eval(&program).unwrap()),
                "\x1b[33mtrue\x1b[0m"
            );
        }
        {
            let mut l = Lexer::new("false".to_string());
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(
                format!("{}", eval(&program).unwrap()),
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
            assert_eq!(program.statements.len(), 1);
            assert_eq!(format!("{}", eval(&program).unwrap()), expected);
        }
    }
}
