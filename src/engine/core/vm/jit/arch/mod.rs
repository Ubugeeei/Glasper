pub(super) mod arm64;

pub(crate) trait InlineAssembler<T> {
    fn unsafe_transmute(&mut self) -> extern "C" fn() -> T;
    fn push(&mut self, reg: u64);
    fn pop(&mut self, reg: u64);
    fn add(&mut self, a: u64, b: u64);
    fn sub(&mut self, a: u64, b: u64);
    fn mul(&mut self, a: u64, b: u64);
    fn div(&mut self, a: u64, b: u64);
    fn label(&mut self) -> u64;
    fn jmp(&mut self, label: u64);
    fn call(&mut self, label: u64);
    fn ret(&mut self);
    fn register_map(&self, reg: InlineAssembler64BitsRegister) -> u64;
}

pub(crate) enum InlineAssembler64BitsRegister {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}
