use crate::engine::{core::vm::VirtualMachine, parsing::BuiltinParser};

use super::interface::JSRuntime;

pub struct VMRuntime {
    vm: VirtualMachine,
}

impl VMRuntime {
    pub fn new() -> Self {
        let parser = Box::new(BuiltinParser);
        let vm = VirtualMachine::new(parser);
        VMRuntime { vm }
    }
}

impl JSRuntime for VMRuntime {
    fn run(&mut self, source: String) {
        self.vm.run(source);
    }
}
