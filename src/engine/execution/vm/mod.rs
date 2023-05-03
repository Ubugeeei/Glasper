use crate::engine::ast::Program;

pub(crate) mod bytecodes;
pub(crate) mod codegen;
pub(crate) mod register;
pub(crate) mod vm;

pub(crate) struct Interpreter {
    pub(crate) vm: vm::VM,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Interpreter { vm: vm::VM::new() }
    }

    pub(crate) fn run(&mut self, program: Program) {
        let mut code = codegen::gen(&program);
        self.vm.append_code(&mut code);
        self.vm.run();
    }
}
