#![allow(dead_code)]

use crate::engine::ast::{Expression, Program, Statement};

use super::bytecodes::{
    Bytecodes::{
        Add, Construct, Div, LdaContextSlot, Mod, Mov, Mul, Pop, Push, StaContextSlot, Sub,
    },
    RName::{R0, R1},
};

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
            gen_expression(&stmt.value, code);

            code.extend(&[StaContextSlot]);
            let name = stmt.name.as_bytes();
            let len_bytes = (name.len() as i64).to_le_bytes();
            code.extend(len_bytes);
            code.extend(name);
        }
        _ => todo!(),
    }
}

fn gen_expression(expr: &Expression, code: &mut Vec<u8>) {
    match expr {
        Expression::Number(literal) => {
            gen_number(*literal, code);
            code.extend_from_slice(&[Pop, R0]);
            code.extend_from_slice(&[Construct]); // r0 = created object ptr
            code.extend_from_slice(&[Push, R0]);
        }
        Expression::Binary(expr) => match expr.operator.as_str() {
            "+" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R0]);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Add, R0, R1]);
                code.extend_from_slice(&[Push, R0]);
            }
            "-" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R0]);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Sub, R0, R1]);
                code.extend_from_slice(&[Push, R0]);
            }
            "*" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R0]);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Mul, R0, R1]);
                code.extend_from_slice(&[Push, R0]);
            }
            "/" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R0]);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Div, R0, R1]);
                code.extend_from_slice(&[Push, R0]);
            }
            "%" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R0]);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Mod, R0, R1]);
                code.extend_from_slice(&[Push, R0]);
            }
            _ => todo!(),
        },
        Expression::Identifier(name) => {
            code.extend_from_slice(&[LdaContextSlot]);
            let name = name.as_bytes();
            let len_bytes = (name.len() as i64).to_le_bytes();
            code.extend(len_bytes);
            code.extend(name);
            code.extend_from_slice(&[Push, R0]);
        }
        _ => todo!(),
    }
}

fn gen_number(n: f64, code: &mut Vec<u8>) {
    let n = n as i64;

    code.push(Mov);
    code.push(R0);
    code.push(((n) & 0xff_i64) as u8);
    code.push(((n >> 8) & 0xff_i64) as u8);
    code.push(((n >> 16) & 0xff_i64) as u8);
    code.push(((n >> 24) & 0xff_i64) as u8);
    code.push(((n >> 32) & 0xff_i64) as u8);
    code.push(((n >> 40) & 0xff_i64) as u8);
    code.push(((n >> 48) & 0xff_i64) as u8);
    code.push(((n >> 56) & 0xff_i64) as u8);

    code.push(Push);
    code.push(R0);
}
