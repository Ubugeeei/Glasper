#![allow(dead_code)]

use crate::engine::parsing::{lexer, parser::Parser};

use self::bytecodes::RName;

use super::legacy_object::{JSNumber, RuntimeObject};

pub(crate) mod bytecodes;
pub(crate) mod codegen;
pub(crate) mod context;
pub(crate) mod heap;
pub(crate) mod register;
#[allow(clippy::module_inception)]
pub(crate) mod vm;

pub(crate) struct Interpreter {
    pub(crate) vm: vm::VM,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Interpreter { vm: vm::VM::new() }
    }

    pub(crate) fn run(&mut self, source: String) -> RuntimeObject {
        let mut lexer = lexer::Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();
        let mut code = codegen::gen(&program);
        self.vm.append_code(&mut code);
        self.vm.run();
        self.get_js_value()
    }

    fn get_js_value(&self) -> RuntimeObject {
        let r1 = self.vm.get_reg_v(RName::R0);
        RuntimeObject::Number(JSNumber::new(r1 as f64))
    }
}