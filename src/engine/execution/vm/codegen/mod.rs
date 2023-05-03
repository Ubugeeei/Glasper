use crate::engine::ast::{Expression, Program, Statement};

use super::{bytecodes::Bytecodes, register::RName};

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
            gen_number(*literal, code);
        }
        _ => todo!(),
    }
}

fn gen_number(n: f64, code: &mut Vec<u8>) {
    code.push(Bytecodes::Mov);
    code.push(RName::R1);

    let n = n as i64;
    code.push(((n >> 0) & 0xff as i64) as u8);
    code.push(((n >> 8) & 0xff as i64) as u8);
    code.push(((n >> 16) & 0xff as i64) as u8);
    code.push(((n >> 24) & 0xff as i64) as u8);
    code.push(((n >> 32) & 0xff as i64) as u8);
    code.push(((n >> 40) & 0xff as i64) as u8);
    code.push(((n >> 48) & 0xff as i64) as u8);
    code.push(((n >> 56) & 0xff as i64) as u8);
}
