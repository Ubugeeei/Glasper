#![allow(dead_code)]

use std::io::Error;

use crate::core::{
    object::def::{GNumber, GUndefined, Object},
    parse::ast::{Expression, Program, Statement},
};

pub fn eval(_program: &Program) -> Result<Object, Error> {
    let mut result = Object::Undefined(GUndefined);
    for statement in &_program.statements {
        result = eval_statement(statement)?;
    }
    Ok(result)
}

pub fn eval_statement(statement: &Statement) -> Result<Object, Error> {
    match statement {
        Statement::Expression(expr) => eval_expression(expr),
        _ => Ok(Object::Undefined(GUndefined)),
    }
}

pub fn eval_expression(expr: &Expression) -> Result<Object, Error> {
    match expr {
        Expression::Integer(i) => Ok(Object::Number(GNumber { value: *i })),
        _ => Ok(Object::Undefined(GUndefined)),
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
}
