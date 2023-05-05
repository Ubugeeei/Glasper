#![allow(dead_code)]

use crate::engine::ast::{Expression, Program, Statement};

use super::bytecodes::{
    Bytecodes::{
        Add, Div, LdaContextSlot, LdaSmi, LdaUndefined, Mod, Mul, Pop, Push, Return,
        StaContextSlot, Sub,
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
            let name = stmt.name.as_bytes();
            let len_bytes = (name.len() as i64).to_le_bytes();

            gen_expression(&stmt.value, code);
            code.extend_from_slice(&[&[StaContextSlot], &len_bytes[0..], name].concat());
            code.extend(&[LdaUndefined]);
            code.extend(&[Return]);
        }
        _ => todo!(),
    }
}

fn gen_expression(expr: &Expression, code: &mut Vec<u8>) {
    match expr {
        Expression::Undefined => {
            code.extend(&[LdaUndefined]);
        }
        Expression::Number(literal) => {
            code.extend_from_slice(&[&[LdaSmi], &into_bytes(*literal)[0..]].concat());
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
            let name = name.as_bytes();
            let len_bytes = (name.len() as i64).to_le_bytes();

            code.extend_from_slice(&[&[LdaContextSlot], &len_bytes[0..], name].concat());
            code.extend_from_slice(&[Push, R0]);
        }
        _ => todo!(),
    }
}

fn into_bytes(n: f64) -> [u8; 8] {
    [
        ((n as i64) & 0xff_i64) as u8,
        ((n as i64 >> 8) & 0xff_i64) as u8,
        ((n as i64 >> 16) & 0xff_i64) as u8,
        ((n as i64 >> 24) & 0xff_i64) as u8,
        ((n as i64 >> 32) & 0xff_i64) as u8,
        ((n as i64 >> 40) & 0xff_i64) as u8,
        ((n as i64 >> 48) & 0xff_i64) as u8,
        ((n as i64 >> 56) & 0xff_i64) as u8,
    ]
}
