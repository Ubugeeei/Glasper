#![allow(dead_code)]

use crate::engine::parsing::{lexer, parser::Parser};

pub(crate) mod bytecodes;
pub(crate) mod codegen;
pub(crate) mod constant_table;
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

    pub(crate) fn run(&mut self, source: String) {
        let mut lexer = lexer::Lexer::new(source);
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();
        let mut code = codegen::gen(&program);
        self.vm.append_code(&mut code);
        self.vm.run();
    }
}
