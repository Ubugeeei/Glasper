use crate::engine::ast::{Expression, Program, Statement};

use super::bytecodes::Bytecodes;

pub fn gen(program: &Program) -> Vec<u8> {
    let mut code: Vec<u8> = Vec::new();
    for statement in program.statements.iter() {
        gen_statement(statement, &mut code);
    }
    code
}

fn gen_statement(statement: &Statement, code: &mut Vec<u8>) {
    match statement {
        Statement::Expression(expr) => {
            gen_expression(expr, code);
        }
        _ => todo!(),
    }
}

fn gen_expression(expr: &Expression, code: &mut Vec<u8>) {
    match expr {
        Expression::Number(literal) => {
            gen_number(literal, code);
        }
        _ => todo!(),
    }
}

fn gen_number(literal: &f64, code: &mut Vec<u8>) {
    code.push(Bytecodes::Push);
    let bytes = literal.to_be_bytes();
    for byte in bytes.iter() {
        code.push(*byte);
    }
}
