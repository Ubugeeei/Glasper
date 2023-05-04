#![allow(dead_code)]

use crate::engine::ast::{Expression, Program, Statement};

use super::bytecodes::{Bytecodes, RName};

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
        Statement::Let(stmt) => {
            // TODO: other types
            gen_expression(&stmt.value, code);
            code.extend_from_slice(&[Bytecodes::Pop, RName::R1]);
            code.extend_from_slice(&[Bytecodes::Construct, RName::R1]); // r1 = created js-object ptr

            code.extend(&[Bytecodes::Declare]);
            let name = stmt.name.as_bytes();
            let len_bytes = (name.len() as i64).to_le_bytes();
            code.extend(len_bytes);
            code.extend(name);
            code.extend([RName::R1]);
        }
        _ => todo!(),
    }
}

fn gen_expression(expr: &Expression, code: &mut Vec<u8>) {
    match expr {
        Expression::Number(literal) => {
            gen_number(*literal, code);
        }
        Expression::Binary(expr) => match expr.operator.as_str() {
            "+" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R1]);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Add, RName::R1, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Push, RName::R1]);
            }
            "-" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R1]);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Sub, RName::R1, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Push, RName::R1]);
            }
            "*" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R1]);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Mul, RName::R1, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Push, RName::R1]);
            }
            "/" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R1]);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Div, RName::R1, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Push, RName::R1]);
            }
            "%" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R1]);
                code.extend_from_slice(&[Bytecodes::Pop, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Mod, RName::R1, RName::R2]);
                code.extend_from_slice(&[Bytecodes::Push, RName::R1]);
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn gen_number(n: f64, code: &mut Vec<u8>) {
    code.push(Bytecodes::Mov);
    code.push(RName::R1);

    let n = n as i64;
    code.push(((n) & 0xff_i64) as u8);
    code.push(((n >> 8) & 0xff_i64) as u8);
    code.push(((n >> 16) & 0xff_i64) as u8);
    code.push(((n >> 24) & 0xff_i64) as u8);
    code.push(((n >> 32) & 0xff_i64) as u8);
    code.push(((n >> 40) & 0xff_i64) as u8);
    code.push(((n >> 48) & 0xff_i64) as u8);
    code.push(((n >> 56) & 0xff_i64) as u8);

    code.push(Bytecodes::Push);
    code.push(RName::R1);
}
