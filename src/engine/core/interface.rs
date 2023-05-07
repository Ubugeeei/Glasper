use super::{
    host::{api::Context, HostInterpreter},
    vm::VirtualMachine,
};

pub trait GlasperEngine {
    fn run(&mut self, source: String);
}

pub struct GlasperEngineBuilder;
impl GlasperEngineBuilder {
    pub fn build_vm() -> impl GlasperEngine {
        VirtualMachine::new()
    }

    pub fn build_host(ctx: &mut Context) -> impl GlasperEngine + '_ {
        HostInterpreter::new(ctx)
    }
}
