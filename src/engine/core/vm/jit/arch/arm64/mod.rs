#![allow(dead_code)]

use super::{InlineAssembler, InlineAssembler64BitsRegister};

pub(crate) struct Arm64InlineAssembler {
    codes: Vec<u8>,
}

impl Arm64InlineAssembler {
    pub(crate) fn new() -> Self {
        Arm64InlineAssembler { codes: vec![] }
    }
}

impl<T> InlineAssembler<T> for Arm64InlineAssembler {
    fn unsafe_transmute(&mut self) -> extern "C" fn() -> T {
        unsafe {
            let ptr = self.codes.as_ptr() as *const T;
            std::mem::transmute(ptr)
        }
    }

    fn push(&mut self, _reg: u64) {
        todo!()
    }

    fn pop(&mut self, _reg: u64) {
        todo!()
    }

    fn add(&mut self, _a: u64, _b: u64) {
        todo!()
    }

    fn sub(&mut self, _a: u64, _b: u64) {
        todo!()
    }

    fn mul(&mut self, _a: u64, _b: u64) {
        todo!()
    }

    fn div(&mut self, _a: u64, _b: u64) {
        todo!()
    }

    fn label(&mut self) -> u64 {
        todo!()
    }

    fn jmp(&mut self, _label: u64) {
        todo!()
    }

    fn call(&mut self, _label: u64) {
        todo!()
    }

    fn ret(&mut self) {
        todo!()
    }

    fn register_map(&self, reg: InlineAssembler64BitsRegister) -> u64 {
        match reg {
            InlineAssembler64BitsRegister::R0 => Registers::X0,
            InlineAssembler64BitsRegister::R1 => Registers::X1,
            InlineAssembler64BitsRegister::R2 => Registers::X2,
            InlineAssembler64BitsRegister::R3 => Registers::X3,
            InlineAssembler64BitsRegister::R4 => Registers::X4,
            InlineAssembler64BitsRegister::R5 => Registers::X5,
            InlineAssembler64BitsRegister::R6 => Registers::X6,
            InlineAssembler64BitsRegister::R7 => Registers::X7,
        }
    }
}

#[allow(non_snake_case)]
mod Registers {
    pub const X0: u64 = 0;
    pub const X1: u64 = 1;
    pub const X2: u64 = 2;
    pub const X3: u64 = 3;
    pub const X4: u64 = 4;
    pub const X5: u64 = 5;
    pub const X6: u64 = 6;
    pub const X7: u64 = 7;
}
