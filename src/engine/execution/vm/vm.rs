#![allow(dead_code)]

use super::{
    bytecodes::Bytecodes,
    register::{RName, Register},
};

pub(crate) struct VM {
    register: Register,
    stack: Vec<i64>,

    pc: usize,
    code: Vec<u8>,
}

impl VM {
    pub(crate) fn new() -> Self {
        Self {
            register: Register::new(),
            stack: Vec::new(),
            pc: 0,
            code: Vec::new(),
        }
    }

    pub(crate) fn run(&mut self) {
        // print code as hex
        println!("byte codes:");
        for (i, byte) in self.code.iter().enumerate() {
            if i % 16 == 0 {
                print!("\x1b[30m{:08x}:\x1b[0m     ", i / 16);
                print!("\x1b[30m{:02x}\x1b[0m ", byte);
            } else {
                print!("\x1b[30m{:02x}\x1b[0m ", byte);
            }
            if i % 16 == 15 {
                println!();
            }
        }
        println!();

        loop {
            let opcode = self.fetch();

            match opcode {
                Bytecodes::Mov => {
                    let r = self.fetch();
                    let v = self.fetch_int64();
                    self.mov(r, v);
                }
                Bytecodes::Push => {
                    let r = self.fetch();
                    self.push(r);
                }
                Bytecodes::Pop => {
                    let r = self.fetch();
                    self.pop(r);
                }

                Bytecodes::Add => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.addi(r1, self.get_reg_v(r2))
                }
                Bytecodes::Sub => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.subi(r1, self.get_reg_v(r2))
                }
                Bytecodes::Mul => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.muli(r1, self.get_reg_v(r2))
                }
                Bytecodes::Div => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.divi(r1, self.get_reg_v(r2))
                }
                Bytecodes::Mod => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.modi(r1, self.get_reg_v(r2))
                }

                Bytecodes::AddSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.addi(r1, r2 as i64)
                }
                Bytecodes::SubSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.subi(r1, r2 as i64)
                }
                Bytecodes::MulSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.muli(r1, r2 as i64)
                }
                Bytecodes::DivSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.divi(r1, r2 as i64)
                }
                Bytecodes::ModSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.modi(r1, r2 as i64)
                }
                Bytecodes::Hlt => {
                    break;
                }
                _ => {
                    todo!()
                }
            }
        }
    }

    pub(crate) fn append_code(&mut self, code: &mut Vec<u8>) {
        self.code.append(code);
    }

    fn fetch(&mut self) -> u8 {
        if self.pc < self.code.len() {
            let opcode = self.code[self.pc];
            self.pc += 1;
            opcode
        } else {
            Bytecodes::Hlt
        }
    }

    fn fetch_int64(&mut self) -> i64 {
        let v1 = self.fetch();
        let v2 = self.fetch();
        let v3 = self.fetch();
        let v4 = self.fetch();
        let v5 = self.fetch();
        let v6 = self.fetch();
        let v7 = self.fetch();
        let v8 = self.fetch();

        let v = (v8 as i64) << 56
            | (v7 as i64) << 48
            | (v6 as i64) << 40
            | (v5 as i64) << 32
            | (v4 as i64) << 24
            | (v3 as i64) << 16
            | (v2 as i64) << 8
            | (v1 as i64);

        v
    }

    fn mov(&mut self, r: u8, v: i64) {
        match r {
            RName::R1 => self.register.r1 = v,
            RName::R2 => self.register.r2 = v,
            RName::R3 => self.register.r3 = v,
            RName::R4 => self.register.r4 = v,
            RName::R5 => self.register.r5 = v,
            RName::R6 => self.register.r6 = v,
            RName::R7 => self.register.r7 = v,
            RName::R8 => self.register.r8 = v,
            RName::R9 => self.register.r9 = v,
            RName::R10 => self.register.r10 = v,
            RName::R11 => self.register.r11 = v,
            RName::R12 => self.register.r12 = v,
            RName::R13 => self.register.r13 = v,
            RName::R14 => self.register.r14 = v,
            RName::R15 => self.register.r15 = v,
            RName::R16 => self.register.r16 = v,
            _ => unreachable!(),
        }
    }

    fn push(&mut self, r: u8) {
        match r {
            RName::R1 => self.stack.push(self.register.r1),
            RName::R2 => self.stack.push(self.register.r2),
            RName::R3 => self.stack.push(self.register.r3),
            RName::R4 => self.stack.push(self.register.r4),
            RName::R5 => self.stack.push(self.register.r5),
            RName::R6 => self.stack.push(self.register.r6),
            RName::R7 => self.stack.push(self.register.r7),
            RName::R8 => self.stack.push(self.register.r8),
            RName::R9 => self.stack.push(self.register.r9),
            RName::R10 => self.stack.push(self.register.r10),
            RName::R11 => self.stack.push(self.register.r11),
            RName::R12 => self.stack.push(self.register.r12),
            RName::R13 => self.stack.push(self.register.r13),
            RName::R14 => self.stack.push(self.register.r14),
            RName::R15 => self.stack.push(self.register.r15),
            RName::R16 => self.stack.push(self.register.r16),
            _ => unreachable!(),
        }
    }

    fn pop(&mut self, r: u8) {
        match r {
            RName::R1 => self.register.r1 = self.stack.pop().unwrap(),
            RName::R2 => self.register.r2 = self.stack.pop().unwrap(),
            RName::R3 => self.register.r3 = self.stack.pop().unwrap(),
            RName::R4 => self.register.r4 = self.stack.pop().unwrap(),
            RName::R5 => self.register.r5 = self.stack.pop().unwrap(),
            RName::R6 => self.register.r6 = self.stack.pop().unwrap(),
            RName::R7 => self.register.r7 = self.stack.pop().unwrap(),
            RName::R8 => self.register.r8 = self.stack.pop().unwrap(),
            RName::R9 => self.register.r9 = self.stack.pop().unwrap(),
            RName::R10 => self.register.r10 = self.stack.pop().unwrap(),
            RName::R11 => self.register.r11 = self.stack.pop().unwrap(),
            RName::R12 => self.register.r12 = self.stack.pop().unwrap(),
            RName::R13 => self.register.r13 = self.stack.pop().unwrap(),
            RName::R14 => self.register.r14 = self.stack.pop().unwrap(),
            RName::R15 => self.register.r15 = self.stack.pop().unwrap(),
            RName::R16 => self.register.r16 = self.stack.pop().unwrap(),
            _ => unreachable!(),
        }
    }

    fn addi(&mut self, r: u8, v: i64) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 + v);
    }

    fn subi(&mut self, r: u8, v: i64) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 - v);
    }

    fn muli(&mut self, r: u8, v: i64) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 * v);
    }

    fn divi(&mut self, r: u8, v: i64) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 / v);
    }

    fn modi(&mut self, r: u8, v: i64) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 % v);
    }

    pub(crate) fn get_reg_v(&self, r: u8) -> i64 {
        match r {
            RName::R1 => self.register.r1,
            RName::R2 => self.register.r2,
            RName::R3 => self.register.r3,
            RName::R4 => self.register.r4,
            RName::R5 => self.register.r5,
            RName::R6 => self.register.r6,
            RName::R7 => self.register.r7,
            RName::R8 => self.register.r8,
            RName::R9 => self.register.r9,
            RName::R10 => self.register.r10,
            RName::R11 => self.register.r11,
            RName::R12 => self.register.r12,
            RName::R13 => self.register.r13,
            RName::R14 => self.register.r14,
            RName::R15 => self.register.r15,
            RName::R16 => self.register.r16,
            _ => unreachable!(),
        }
    }
}
