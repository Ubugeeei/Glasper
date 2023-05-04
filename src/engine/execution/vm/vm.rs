#![allow(dead_code)]

use std::ptr::NonNull;

use crate::engine::execution::objects::{js_number::JSNumber, objects::Object};

use super::{
    bytecodes::{Bytecodes, RName},
    context::ExecutionContext,
    heap::Heap,
    register::Register,
};

pub struct VM {
    register: Register,
    pc: usize,
    code: Vec<u8>,
    stack: Vec<usize>,
    execution_context: ExecutionContext,
    pub(crate) heap: Heap,
}

impl VM {
    pub(crate) fn new() -> Self {
        Self {
            register: Register::new(),
            pc: 0,
            stack: Vec::new(),
            code: Vec::new(),
            execution_context: ExecutionContext::new(),
            heap: Heap::new(1024 * 1024),
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
                    let v = self.fetch_usize();
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
                    self.addi(r1, r2 as usize)
                }
                Bytecodes::SubSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.subi(r1, r2 as usize)
                }
                Bytecodes::MulSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.muli(r1, r2 as usize)
                }
                Bytecodes::DivSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.divi(r1, r2 as usize)
                }
                Bytecodes::ModSmi => {
                    let r1 = self.fetch();
                    let r2 = self.fetch();
                    self.modi(r1, r2 as usize)
                }
                Bytecodes::Hlt => {
                    break;
                }

                Bytecodes::Construct => {
                    // TODO: other types
                    let reg_v_h = self.get_reg_v(RName::R0);
                    let reg_v_l = self.get_reg_v(RName::R1);
                    let reg_v = ((reg_v_h as u64) << 32) | (reg_v_l as u64);

                    let mut o = self.heap.alloc().unwrap();
                    let js_value = JSNumber::create(reg_v as f64, &mut o, self);
                    let raw_ptr = js_value.ptr.as_ptr() as usize;
                    self.mov(RName::R0, raw_ptr);
                }

                Bytecodes::StaContextSlot => {
                    let name = self.fetch_string();
                    let reg_v = self.get_reg_v(RName::R0);
                    let ptr = NonNull::new(reg_v as *mut Object).unwrap();
                    self.execution_context
                        .context
                        .clone()
                        .borrow_mut()
                        .set(name, ptr);
                }
                Bytecodes::LdaContextSlot => {
                    let name = self.fetch_string();
                    // TODO: undefined
                    let ptr = self
                        .execution_context
                        .context
                        .clone()
                        .borrow()
                        .get(&name)
                        .unwrap();

                    let raw_ptr = ptr.as_ptr() as usize;
                    self.mov(RName::R0, raw_ptr);
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

    pub(crate) fn display(&self) {
        let ptr = self.get_reg_v(RName::R0);
        let o = Object::from_row_ptr(ptr);
        let js_value = o.as_js_object_ref();
        println!("{}", js_value);
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

    fn fetch_usize(&mut self) -> usize {
        let v1 = self.fetch();
        let v2 = self.fetch();
        let v3 = self.fetch();
        let v4 = self.fetch();

        (v4 as usize) << 24 | (v3 as usize) << 16 | (v2 as usize) << 8 | (v1 as usize)
    }

    fn fetch_string(&mut self) -> String {
        let len = self.fetch_usize();
        let mut s = String::new();
        for _ in 0..len {
            let c = self.fetch();
            s.push(c as char);
        }
        s
    }

    fn mov(&mut self, r: u8, v: usize) {
        match r {
            RName::R0 => self.register.r0 = v,
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
            _ => unreachable!(),
        }
    }

    fn push(&mut self, r: u8) {
        match r {
            RName::R0 => self.stack.push(self.register.r0),
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
            _ => unreachable!(),
        }
    }

    fn pop(&mut self, r: u8) {
        match r {
            RName::R0 => self.register.r0 = self.stack.pop().unwrap(),
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
            _ => unreachable!(),
        }
    }

    fn addi(&mut self, r: u8, v: usize) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 + v);
    }

    fn subi(&mut self, r: u8, v: usize) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 - v);
    }

    fn muli(&mut self, r: u8, v: usize) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 * v);
    }

    fn divi(&mut self, r: u8, v: usize) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 / v);
    }

    fn modi(&mut self, r: u8, v: usize) {
        let v1 = self.get_reg_v(r);
        self.mov(r, v1 % v);
    }

    pub(crate) fn get_reg_v(&self, r: u8) -> usize {
        match r {
            RName::R0 => self.register.r0,
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
            _ => unreachable!(),
        }
    }
}
