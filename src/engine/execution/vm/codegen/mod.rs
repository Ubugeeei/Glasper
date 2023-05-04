#![allow(dead_code)]

use crate::engine::ast::{Expression, Program, Statement};

use super::bytecodes::{
    Bytecodes::{Add, Construct, Declare, Div, Mod, Mov, Mul, Pop, Push, Sub},
    RName::{R1, R2},
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
            // TODO: other types
            gen_expression(&stmt.value, code);
            code.extend_from_slice(&[Pop, R1]);
            code.extend_from_slice(&[Construct, R1]); // r1 = created js-object ptr

            code.extend(&[Declare]);
            let name = stmt.name.as_bytes();
            let len_bytes = (name.len() as i64).to_le_bytes();
            code.extend(len_bytes);
            code.extend(name);
            code.extend([R1]);
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
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Pop, R2]);
                code.extend_from_slice(&[Add, R1, R2]);
                code.extend_from_slice(&[Push, R1]);
            }
            "-" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Pop, R2]);
                code.extend_from_slice(&[Sub, R1, R2]);
                code.extend_from_slice(&[Push, R1]);
            }
            "*" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Pop, R2]);
                code.extend_from_slice(&[Mul, R1, R2]);
                code.extend_from_slice(&[Push, R1]);
            }
            "/" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Pop, R2]);
                code.extend_from_slice(&[Div, R1, R2]);
                code.extend_from_slice(&[Push, R1]);
            }
            "%" => {
                gen_expression(&expr.left, code);
                gen_expression(&expr.right, code);
                code.extend_from_slice(&[Pop, R1]);
                code.extend_from_slice(&[Pop, R2]);
                code.extend_from_slice(&[Mod, R1, R2]);
                code.extend_from_slice(&[Push, R1]);
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn gen_number(n: f64, code: &mut Vec<u8>) {
    code.push(Mov);
    code.push(R1);

    let n = n as i64;
    code.push(((n) & 0xff_i64) as u8);
    code.push(((n >> 8) & 0xff_i64) as u8);
    code.push(((n >> 16) & 0xff_i64) as u8);
    code.push(((n >> 24) & 0xff_i64) as u8);
    code.push(((n >> 32) & 0xff_i64) as u8);
    code.push(((n >> 40) & 0xff_i64) as u8);
    code.push(((n >> 48) & 0xff_i64) as u8);
    code.push(((n >> 56) & 0xff_i64) as u8);

    code.push(Push);
    code.push(R1);
}
