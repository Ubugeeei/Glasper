#![allow(dead_code)]

use crate::engine::ast::{Expression, Program, Statement};

use super::{
    bytecodes::{
        Bytecodes::{
            Add, Div, LdaConstant, LdaContextSlot, LdaSmi, LdaUndefined, Mod, Mul, Pop, Push,
            Return, StaContextSlot, Sub,
        },
        RName::{R0, R1},
    },
    constant_table::ConstantTable,
};

pub struct CodeGenerator<'a> {
    code: Vec<u8>,
    constant_table: &'a mut ConstantTable,
}

impl<'a> CodeGenerator<'a> {
    pub(super) fn new(constant_table: &'a mut ConstantTable) -> Self {
        CodeGenerator {
            code: Vec::new(),
            constant_table,
        }
    }

    pub(super) fn gen(&mut self, program: &Program) -> Vec<u8> {
        for statement in program.statements.iter() {
            self.gen_statement(statement);
        }
        self.code.clone()
    }

    fn gen_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Expression(expr) => {
                self.gen_expression(expr);
            }

            Statement::Let(stmt) => {
                let name = stmt.name.as_bytes();
                let len_bytes = (name.len() as i64).to_le_bytes();

                self.gen_expression(&stmt.value);
                self.code
                    .extend_from_slice(&[&[StaContextSlot], &len_bytes[0..], name].concat());
                self.code.extend(&[LdaUndefined]);
                self.code.extend(&[Return]);
            }
            _ => todo!(),
        }
    }

    fn gen_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Undefined => {
                self.code.extend(&[LdaUndefined]);
            }
            Expression::Number(literal) => {
                self.code
                    .extend_from_slice(&[&[LdaSmi], &self.into_bytes(*literal)[0..]].concat());
                self.code.extend_from_slice(&[Push, R0]);
            }
            Expression::String(literal) => {
                // TODO: constant_table
                // self.code.extend_from_slice(&[LdaConstant]);
                // self.code.extend_from_slice(&[Push, R0]);
            }
            Expression::Binary(expr) => match expr.operator.as_str() {
                "+" => {
                    self.gen_expression(&expr.left);
                    self.gen_expression(&expr.right);
                    self.code.extend_from_slice(&[Pop, R0]);
                    self.code.extend_from_slice(&[Pop, R1]);
                    self.code.extend_from_slice(&[Add, R0, R1]);
                    self.code.extend_from_slice(&[Push, R0]);
                }
                "-" => {
                    self.gen_expression(&expr.left);
                    self.gen_expression(&expr.right);
                    self.code.extend_from_slice(&[Pop, R0]);
                    self.code.extend_from_slice(&[Pop, R1]);
                    self.code.extend_from_slice(&[Sub, R0, R1]);
                    self.code.extend_from_slice(&[Push, R0]);
                }
                "*" => {
                    self.gen_expression(&expr.left);
                    self.gen_expression(&expr.right);
                    self.code.extend_from_slice(&[Pop, R0]);
                    self.code.extend_from_slice(&[Pop, R1]);
                    self.code.extend_from_slice(&[Mul, R0, R1]);
                    self.code.extend_from_slice(&[Push, R0]);
                }
                "/" => {
                    self.gen_expression(&expr.left);
                    self.gen_expression(&expr.right);
                    self.code.extend_from_slice(&[Pop, R0]);
                    self.code.extend_from_slice(&[Pop, R1]);
                    self.code.extend_from_slice(&[Div, R0, R1]);
                    self.code.extend_from_slice(&[Push, R0]);
                }
                "%" => {
                    self.gen_expression(&expr.left);
                    self.gen_expression(&expr.right);
                    self.code.extend_from_slice(&[Pop, R0]);
                    self.code.extend_from_slice(&[Pop, R1]);
                    self.code.extend_from_slice(&[Mod, R0, R1]);
                    self.code.extend_from_slice(&[Push, R0]);
                }
                _ => todo!(),
            },
            Expression::Identifier(name) => {
                let name = name.as_bytes();
                let len_bytes = (name.len() as i64).to_le_bytes();

                self.code
                    .extend_from_slice(&[&[LdaContextSlot], &len_bytes[0..], name].concat());
                self.code.extend_from_slice(&[Push, R0]);
            }
            _ => todo!(),
        }
    }

    fn into_bytes(&self, n: f64) -> [u8; 8] {
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
}
