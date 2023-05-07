use super::{host::js::HostJSRuntime, vm::VMRuntime};

pub trait JSRuntime {
    fn run(&mut self, source: String);
}

pub struct JSRuntimeBuilder;
impl JSRuntimeBuilder {
    pub fn build(vm: bool) -> Box<dyn JSRuntime> {
        if vm {
            Box::new(Self::build_vm())
        } else {
            Box::new(Self::build_host())
        }
    }

    fn build_vm() -> impl JSRuntime {
        VMRuntime::new()
    }

    fn build_host() -> impl JSRuntime {
        HostJSRuntime::new()
    }
}
