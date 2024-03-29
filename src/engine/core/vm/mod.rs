#![allow(dead_code)]

use crate::engine::parsing::Parser;

use self::{
    bytecodes::{Bytecodes, RName},
    codegen::CodeGenerator,
    constant_table::ConstantTable,
    context::ExecutionContext,
    heap::Heap,
    objects::{
        js_number::JSNumber,
        js_object::{JSObject, JSType},
        js_string::JSString,
    },
    register::Register,
};

use std::fmt::Display;

pub(crate) mod bytecodes;
pub(crate) mod codegen;
pub(crate) mod constant_table;
pub(crate) mod context;
pub(crate) mod heap;
pub(crate) mod jit;
pub(crate) mod objects;
pub(crate) mod register;

enum VMErrorKind {
    Type,
    Reference,
    Syntax,
    Range,
    Eval,
    Internal,
}
pub(crate) struct VMError {
    kind: VMErrorKind,
    message: String,
}
impl VMError {
    fn new(kind: VMErrorKind, message: String) -> Self {
        VMError { kind, message }
    }
}
impl Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            match self.kind {
                VMErrorKind::Type => "TypeError",
                VMErrorKind::Reference => "ReferenceError",
                VMErrorKind::Syntax => "SyntaxError",
                VMErrorKind::Range => "RangeError",
                VMErrorKind::Eval => "EvalError",
                VMErrorKind::Internal => "InternalError",
            },
            self.message
        )
    }
}

pub(crate) struct VirtualMachine {
    execution_context: ExecutionContext,
    pub(crate) constant_table: ConstantTable,
    parser: Box<dyn Parser>,

    register: Register,
    pc: usize,
    code: Vec<u8>,
    stack: Vec<i64>,
    pub(crate) heap: Heap,
}

impl VirtualMachine {
    pub(crate) fn new(parser: Box<dyn Parser>) -> Self {
        let execution_context = ExecutionContext::new();
        let mut heap = Heap::new(1024 * 1024);

        // create global objects
        let base_obj = heap.alloc().unwrap();
        base_obj._type = JSType::Undefined;
        execution_context
            .context
            .borrow_mut()
            .set("undefined".to_string(), base_obj.as_raw_ptr());

        Self {
            execution_context,
            constant_table: ConstantTable::new(),
            parser,
            register: Register::new(),
            pc: 0,
            stack: Vec::new(),
            code: Vec::new(),
            heap,
        }
    }
}

/// core impl
impl VirtualMachine {
    pub fn run(&mut self, source: String) {
        match &*source {
            "%PrintDump()" => {
                self.print_dump();
            }
            "%PrintIr()" => {
                self.print_ir();
            }
            "%PrintBytes()" => {
                self.print_bytecode();
            }
            _ => {
                let program = self.parser.parse(source);
                let mut codegen = CodeGenerator::new(&mut self.constant_table);
                let mut code = codegen.gen(&program);
                self.code.append(&mut code);

                if let Err(e) = self.interpret() {
                    println!("{}", e);
                } else {
                    self.print_current_expr();
                };
            }
        };
    }

    fn interpret(&mut self) -> Result<(), VMError> {
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

                Bytecodes::LdaUndefined => self.load_undefined()?,
                Bytecodes::LdaSmi => {
                    let v = self.fetch_i64();
                    if let Some(base_obj) = self.heap.alloc() {
                        let num_obj = JSNumber::create(v as f64, base_obj, self);
                        let ptr = num_obj.as_raw_ptr();
                        self.mov(RName::R0, ptr);
                    } else {
                        return Err(VMError::new(
                            VMErrorKind::Internal,
                            "allocation failed".to_string(),
                        ));
                    }
                }
                Bytecodes::LdaConstant => {
                    let id = self.fetch_i64();
                    if let Some(base_obj) = self.heap.alloc() {
                        let s = self.constant_table.get(id as u32).clone();
                        let str_obj = JSString::create(s, base_obj, self);
                        self.mov(RName::R0, str_obj.as_raw_ptr());
                    } else {
                        return Err(VMError::new(
                            VMErrorKind::Internal,
                            "allocation failed".to_string(),
                        ));
                    }
                }
                Bytecodes::LdaContextSlot => {
                    let name = self.fetch_string();
                    if let Some(raw_ptr) =
                        self.execution_context.context.clone().borrow().get(&name)
                    {
                        self.mov(RName::R0, raw_ptr);
                    } else {
                        return Err(VMError::new(
                            VMErrorKind::Reference,
                            format!("{} is not defined", name),
                        ));
                    }
                }
                Bytecodes::GetNamedProperty => {
                    let reg = self.fetch();
                    let obj_ptr = self.get_reg_v(reg);
                    let id = self.fetch_i64();
                    let name = self.constant_table.get(id as u32).clone();
                    let obj = JSObject::from_raw_ptr(obj_ptr);
                    if let Some(prop) = obj.get(&name) {
                        self.mov(RName::R0, prop.as_raw_ptr());
                    } else {
                        let _ = self.load_undefined();
                    }
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

                Bytecodes::CallProperty => {
                    // get callee function
                    let callee_pointer_reg = self.fetch();
                    let callee_pointer = self.get_reg_v(callee_pointer_reg);
                    let callee = JSObject::from_raw_ptr(callee_pointer);
                    let callee_fn = match callee._type {
                        JSType::NativeFunction(f) => f,
                        // TODO: JSFunction,
                        _ => {
                            return Err(VMError::new(
                                VMErrorKind::Type,
                                "callee is not a function".to_string(),
                            ))
                        }
                    };

                    // TODO: arguments
                    // let arg_pointer_reg = self.fetch();
                    // let arg_pointer = self.get_reg_v(arg_pointer_reg);

                    // parent object
                    let parent_obj_pointer_reg = self.fetch();
                    let parent_obj_pointer = self.get_reg_v(parent_obj_pointer_reg);
                    let parent_obj = JSObject::from_raw_ptr_mut(parent_obj_pointer);

                    // call
                    let ret = callee_fn(self, parent_obj, vec![]);
                    self.mov(RName::R0, ret.as_raw_ptr());
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

                _ => todo!(),
            }
        }
        Ok(())
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

    fn load_undefined(&mut self) -> Result<(), VMError> {
        if let Some(raw_ptr) = self
            .execution_context
            .context
            .clone()
            .borrow()
            .get("undefined")
        {
            self.mov(RName::R0, raw_ptr);
            Ok(())
        } else {
            Err(VMError::new(
                VMErrorKind::Internal,
                "internal error".to_string(),
            ))
        }
    }

    fn add(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let l_obj = JSObject::from_raw_ptr(self.get_reg_v(r1));
        let r_obj = JSObject::from_raw_ptr(self.get_reg_v(r2));

        match (&l_obj._type, &r_obj._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 + n2, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
            (JSType::String(s1), JSType::String(s2)) => {
                let base_obj = self.heap.alloc().unwrap();
                let mut s = s2.clone();
                s.push_str(s1);
                let num_obj = JSString::create(s, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
            (JSType::Number(n1), JSType::String(s2)) => {
                let base_obj = self.heap.alloc().unwrap();
                let mut s = s2.clone();
                s.push_str(&n1.to_string());
                let num_obj = JSString::create(s, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
            (JSType::String(s1), JSType::Number(n2)) => {
                let base_obj = self.heap.alloc().unwrap();
                let mut s = n2.to_string();
                s.push_str(s1);
                let num_obj = JSString::create(s, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
            _ => {
                // TODO: string + others
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(f64::NAN, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
        }
    }

    fn sub(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let l_obj = JSObject::from_raw_ptr(self.get_reg_v(r1));
        let r_obj = JSObject::from_raw_ptr(self.get_reg_v(r2));

        match (&l_obj._type, &r_obj._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 - n2, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
            _ => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(f64::NAN, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
        }
    }

    fn mul(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let l_obj = JSObject::from_raw_ptr(self.get_reg_v(r1));
        let r_obj = JSObject::from_raw_ptr(self.get_reg_v(r2));

        match (&l_obj._type, &r_obj._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 * n2, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
            _ => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(f64::NAN, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
        }
    }

    fn div(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let l_obj = JSObject::from_raw_ptr(self.get_reg_v(r1));
        let r_obj = JSObject::from_raw_ptr(self.get_reg_v(r2));

        match (&l_obj._type, &r_obj._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 / n2, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
            _ => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(f64::NAN, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
        }
    }

    fn r#mod(&mut self) {
        let r1 = self.fetch();
        let r2 = self.fetch();
        let l_obj = JSObject::from_raw_ptr(self.get_reg_v(r1));
        let r_obj = JSObject::from_raw_ptr(self.get_reg_v(r2));

        match (&l_obj._type, &r_obj._type) {
            (JSType::Number(n1), JSType::Number(n2)) => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(n1 % n2, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
            _ => {
                let base_obj = self.heap.alloc().unwrap();
                let num_obj = JSNumber::create(f64::NAN, base_obj, self);
                self.mov(RName::R0, num_obj.as_raw_ptr());
            }
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
}

/// printer impl
impl VirtualMachine {
    fn print_current_expr(&self) {
        let ptr = self.get_reg_v(RName::R0);
        let obj = JSObject::from_raw_ptr(ptr);
        println!("{}", obj);
    }

    fn print_bytecode(&self) {
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

    fn print_bytecode_with_ir(&self) {
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

    fn print_ir(&self) {
        let i = self.get_instructions(&self.code);
        for (inst, _) in i {
            print!("\x1b[30m{inst}\x1b[0m ");
            println!();
        }
    }

    fn print_dump(&self) {
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

    fn get_instructions<'b>(&'b self, code: &'b Vec<u8>) -> Vec<(String, &[u8])> {
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
                    res.push(("Hlt".to_string(), &code[i..i + 1]));
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
                    res.push(("Construct".to_string(), &code[i..i + 1]));
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

                Bytecodes::LdaUndefined => {
                    res.push(("LdaUndefined".to_string(), &code[i..i + 1]));
                    i += 1;
                }

                Bytecodes::LdaSmi => {
                    let v = ((code[i + 8] as i64) << 56
                        | (code[i + 7] as i64) << 48
                        | (code[i + 6] as i64) << 40
                        | (code[i + 5] as i64) << 32
                        | (code[i + 4] as i64) << 24
                        | (code[i + 3] as i64) << 16
                        | (code[i + 2] as i64) << 8
                        | (code[i + 1] as i64)) as usize;

                    res.push((format!("LdaSmi {}", v), &code[i..i + 9]));
                    i += 9;
                }

                Bytecodes::GetNamedProperty => {
                    let obj_ptr_reg = code[i + 1];
                    let prop_name_idx = code[i + 2];
                    res.push((
                        format!("GetNamedProperty r{obj_ptr_reg}, [{prop_name_idx}]",),
                        &code[i..i + 3],
                    ));
                    i += 3;
                }

                Bytecodes::CallProperty => {
                    let callee_pointer_reg = code[i + 1];
                    // let arg_count_reg = code[i + 2];
                    let parent_obj_pointer = code[i + 2];

                    res.push((
                        format!("CallProperty r{callee_pointer_reg}, r{parent_obj_pointer}",),
                        &code[i..i + 3],
                    ));
                    i += 3;
                }

                Bytecodes::Return => {
                    res.push(("Return".to_string(), &code[i..i + 1]));
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }

        res
    }
}
