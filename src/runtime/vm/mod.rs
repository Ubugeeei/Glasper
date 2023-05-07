use crate::engine::core::vm::VirtualMachine;

use super::interface::JSRuntime;

pub struct VMRuntime {
    vm: VirtualMachine,
}

impl VMRuntime {
    pub fn new() -> Self {
        let vm = VirtualMachine::new();
        VMRuntime { vm }
    }
}

impl JSRuntime for VMRuntime {
    fn run(&mut self, source: String) {
        self.vm.run(source);
    }
}
