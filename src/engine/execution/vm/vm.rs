#![allow(dead_code)]

use crate::engine::execution::objects::{js_number::JSNumber, js_object::JSType, object::Object};

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
    stack: Vec<i64>,
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

    fn init_internal(&mut self) {
        let mut base_obj = self.heap.alloc().unwrap();
        base_obj.as_js_object_mut()._type = JSType::Undefined;
        self.execution_context
            .context
            .clone()
            .borrow_mut()
            .set("undefined".to_string(), base_obj.raw_ptr());
    }

    pub(crate) fn run(&mut self) {
        self.init_internal();
        loop {
            let opcode = self.fetch();

            match opcode {
                Bytecodes::Mov => {
                    let r = self.fetch();
                    let v = self.fetch_i64();
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
                Bytecodes::Hlt => {
                    break;
                }

                Bytecodes::LdaUndefined => {
                    let raw_ptr = self
                        .execution_context
                        .context
                        .clone()
                        .borrow()
                        .get("undefined")
                        .unwrap();
                    self.mov(RName::R0, raw_ptr);
                }
                Bytecodes::Return => {
                    // TODO: return to called point
                    break;
                }

                // binary operations
                Bytecodes::Add => self.add(),
                Bytecodes::Sub => self.sub(),
                Bytecodes::Mul => self.mul(),
                Bytecodes::Div => self.div(),
                Bytecodes::Mod => self.r#mod(),

                Bytecodes::Construct => {
                    // TODO: other types
                    let reg_v = self.get_reg_v(RName::R0);
                    let mut base_obj = self.heap.alloc().unwrap();
                    let num_obj = JSNumber::create(reg_v as f64, &mut base_obj, self);
                    let raw_ptr = num_obj.raw_ptr();
                    self.mov(RName::R0, raw_ptr);
                }

                Bytecodes::StaContextSlot => {
                    let reg_v = self.get_reg_v(RName::R0);
                    let name = self.fetch_string();
                    self.execution_context
                        .context
                        .clone()
                        .borrow_mut()
                        .set(name, reg_v);
                }
                Bytecodes::LdaContextSlot => {
                    let name = self.fetch_string();
                    // TODO: undefined
                    let raw_ptr = self
                        .execution_context
                        .context
                        .clone()
                        .borrow()
                        .get(&name)
                        .unwrap();
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

    fn fetch(&mut self) -> u8 {
        if self.pc < self.code.len() {
            let opcode = self.code[self.pc];
            self.pc += 1;
            opcode
        } else {
            Bytecodes::Hlt
        }
    }

    fn fetch_i64(&mut self) -> i64 {
        let v0 = self.fetch();
        let v1 = self.fetch();
        let v2 = self.fetch();
        let v3 = self.fetch();
        let v4 = self.fetch();
        let v5 = self.fetch();
        let v6 = self.fetch();
        let v7 = self.fetch();

        (v7 as i64) << 56
            | (v6 as i64) << 48
            | (v5 as i64) << 40
            | (v4 as i64) << 32
            | (v3 as i64) << 24
            | (v2 as i64) << 16
            | (v1 as i64) << 8
            | (v0 as i64)
    }

    fn fetch_string(&mut self) -> String {
        let len = self.fetch_i64();
        let mut s = String::new();
        for _ in 0..len {
            let c = self.fetch();
            s.push(c as char);
        }
        s
    }

    fn mov(&mut self, r: u8, v: i64) {
        match r {
            RName::R0 => self.register.r0 = v,
            RName::R1 => self.register.r1 = v,
            RName::R2 => self.register.r2 = v,
            RName::R3 => self.register.r3 = v,
            RName::R4 => self.register.r4 = v,
            RName::R5 => self.register.r5 = v,
            RName::R6 => self.register.r6 = v,
            RName::R7 => self.register.r7 = v,
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
            _ => unreachable!(),
        }
    }

    fn add(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let o1 = Object::from_row_ptr(self.get_reg_v(r1));
        let o2 = Object::from_row_ptr(self.get_reg_v(r2));
        let jso1 = o1.as_js_object_ref();
        let jso2 = o2.as_js_object_ref();

        match (&jso1._type, &jso2._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let mut base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 + n2, &mut base_obj, self);
                let raw_ptr = num_obj.raw_ptr();
                self.mov(RName::R0, raw_ptr);
            }
            _ => todo!("implement add for other types"),
        }
    }

    fn sub(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let o1 = Object::from_row_ptr(self.get_reg_v(r1));
        let o2 = Object::from_row_ptr(self.get_reg_v(r2));
        let jso1 = o1.as_js_object_ref();
        let jso2 = o2.as_js_object_ref();

        match (&jso1._type, &jso2._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let mut base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 - n2, &mut base_obj, self);
                let raw_ptr = num_obj.raw_ptr();
                self.mov(RName::R0, raw_ptr);
            }
            _ => todo!("implement add for other types"),
        }
    }

    fn mul(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let o1 = Object::from_row_ptr(self.get_reg_v(r1));
        let o2 = Object::from_row_ptr(self.get_reg_v(r2));
        let jso1 = o1.as_js_object_ref();
        let jso2 = o2.as_js_object_ref();

        match (&jso1._type, &jso2._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let mut base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 * n2, &mut base_obj, self);
                let raw_ptr = num_obj.raw_ptr();
                self.mov(RName::R0, raw_ptr);
            }
            _ => todo!("implement add for other types"),
        }
    }

    fn div(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let o1 = Object::from_row_ptr(self.get_reg_v(r1));
        let o2 = Object::from_row_ptr(self.get_reg_v(r2));
        let jso1 = o1.as_js_object_ref();
        let jso2 = o2.as_js_object_ref();

        match (&jso1._type, &jso2._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let mut base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 / n2, &mut base_obj, self);
                let raw_ptr = num_obj.raw_ptr();
                self.mov(RName::R0, raw_ptr);
            }
            _ => todo!("implement add for other types"),
        }
    }

    fn r#mod(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let o1 = Object::from_row_ptr(self.get_reg_v(r1));
        let o2 = Object::from_row_ptr(self.get_reg_v(r2));
        let jso1 = o1.as_js_object_ref();
        let jso2 = o2.as_js_object_ref();

        match (&jso1._type, &jso2._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let mut base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 % n2, &mut base_obj, self);
                let raw_ptr = num_obj.raw_ptr();
                self.mov(RName::R0, raw_ptr);
            }
            _ => todo!("implement add for other types"),
        }
    }

    pub(crate) fn get_reg_v(&self, r: u8) -> i64 {
        match r {
            RName::R0 => self.register.r0,
            RName::R1 => self.register.r1,
            RName::R2 => self.register.r2,
            RName::R3 => self.register.r3,
            RName::R4 => self.register.r4,
            RName::R5 => self.register.r5,
            RName::R6 => self.register.r6,
            RName::R7 => self.register.r7,
            _ => unreachable!(),
        }
    }

    pub(crate) fn print(&self) {
        let ptr = self.get_reg_v(RName::R0);
        let o = Object::from_row_ptr(ptr);
        let js_value = o.as_js_object_ref();
        println!("{}", js_value);
    }

    pub(crate) fn print_bytecode(&self) {
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
    }

    pub(crate) fn print_bytecode_with_ir(&self) {
        for (i, byte) in self.code.iter().enumerate() {
            if i % 16 == 0 {
                print!("\x1b[30m{:?} @    {}:\x1b[0m ", byte as *const u8, i / 16);
                print!("\x1b[30m{:02x}\x1b[0m ", byte);
            } else {
                print!("\x1b[30m{:02x}\x1b[0m ", byte);
            }
            if i % 16 == 15 {
                println!();
            }
        }
        println!();
    }

    pub(crate) fn print_ir(&self) {
        let i = self.get_instructions(&self.code);
        for (inst, _) in i {
            print!("\x1b[30m{inst}\x1b[0m ");
            println!();
        }
    }

    pub(crate) fn print_dump(&self) {
        let i = self.get_instructions(&self.code);
        for (idx, (_, bytes)) in i.iter().enumerate() {
            print!(
                "  \x1b[30m0x{:x} @     0x{idx:08x} : \x1b[0m",
                bytes.as_ptr() as usize
            );
            for byte in bytes.iter() {
                print!("\x1b[30m{:02x}\x1b[0m ", byte);
            }
            println!();
        }
    }

    fn get_instructions<'a>(&'a self, code: &'a Vec<u8>) -> Vec<(String, &[u8])> {
        let mut res: Vec<(String, &[u8])> = Vec::new();
        let mut i = 0;

        while i < code.len() {
            match code[i] {
                Bytecodes::Mov => {
                    let r1 = code[i + 1];

                    let v = (code[i + 9] as i64) << 56
                        | (code[i + 8] as i64) << 48
                        | (code[i + 7] as i64) << 40
                        | (code[i + 6] as i64) << 32
                        | (code[i + 5] as i64) << 24
                        | (code[i + 4] as i64) << 16
                        | (code[i + 3] as i64) << 8
                        | (code[i + 2] as i64);

                    res.push((format!("Mov r{}, {}", r1, v), &code[i..i + 10]));
                    i += 10;
                }

                Bytecodes::Push => {
                    let r1 = code[i + 1];
                    res.push((format!("Push r{}", r1), &code[i..i + 2]));
                    i += 2;
                }
                Bytecodes::Pop => {
                    let r1 = code[i + 1];
                    res.push((format!("Pop r{}", r1), &code[i..i + 2]));
                    i += 2;
                }
                Bytecodes::Hlt => {
                    res.push((format!("Hlt"), &code[i..i + 1]));
                    i += 1;
                }

                Bytecodes::Add => {
                    let r1 = code[i + 1];
                    let r2 = code[i + 2];
                    res.push((format!("Add r{}, r{}", r1, r2), &code[i..i + 3]));
                    i += 3;
                }
                Bytecodes::Sub => {
                    let r1 = code[i + 1];
                    let r2 = code[i + 2];
                    res.push((format!("Sub r{}, r{}", r1, r2), &code[i..i + 3]));
                    i += 3;
                }
                Bytecodes::Mul => {
                    let r1 = code[i + 1];
                    let r2 = code[i + 2];
                    res.push((format!("Mul r{}, r{}", r1, r2), &code[i..i + 3]));
                    i += 3;
                }
                Bytecodes::Div => {
                    let r1 = code[i + 1];
                    let r2 = code[i + 2];
                    res.push((format!("Div r{}, r{}", r1, r2), &code[i..i + 3]));
                    i += 3;
                }
                Bytecodes::Mod => {
                    let r1 = code[i + 1];
                    let r2 = code[i + 2];
                    res.push((format!("Mod r{}, r{}", r1, r2), &code[i..i + 3]));
                    i += 3;
                }

                Bytecodes::Construct => {
                    res.push((format!("Construct"), &code[i..i + 1]));
                    i += 1;
                }

                Bytecodes::StaContextSlot => {
                    let len = ((code[i + 8] as i64) << 56
                        | (code[i + 7] as i64) << 48
                        | (code[i + 6] as i64) << 40
                        | (code[i + 5] as i64) << 32
                        | (code[i + 4] as i64) << 24
                        | (code[i + 3] as i64) << 16
                        | (code[i + 2] as i64) << 8
                        | (code[i + 1] as i64)) as usize;

                    let name = String::from_utf8(code[i + 9..i + 9 + len].to_vec())
                        .unwrap_or_else(|_| String::from(""));

                    res.push((
                        format!("StaContextSlot \"{}\"", name),
                        &code[i..i + 9 + len],
                    ));
                    i += 9 + len;
                }

                Bytecodes::LdaContextSlot => {
                    let len = ((code[i + 8] as i64) << 56
                        | (code[i + 7] as i64) << 48
                        | (code[i + 6] as i64) << 40
                        | (code[i + 5] as i64) << 32
                        | (code[i + 4] as i64) << 24
                        | (code[i + 3] as i64) << 16
                        | (code[i + 2] as i64) << 8
                        | (code[i + 1] as i64)) as usize;

                    let name = String::from_utf8(code[i + 9..i + 9 + len].to_vec())
                        .unwrap_or_else(|_| String::from(""));

                    res.push((
                        format!("LdaContextSlot \"{}\"", name),
                        &code[i..i + 9 + len],
                    ));
                    i += 9 + len;
                }
                _ => {}
            }
        }

        res
    }
}
