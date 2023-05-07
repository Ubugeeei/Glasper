use self::arch::{arm64::Arm64InlineAssembler, InlineAssembler};

mod arch;

pub(crate) fn jit_compile<T>(code: &[u8], isa: Isa) -> extern "C" fn() -> T {
    let mut compiler = JitCompiler::new(code, isa);
    compiler.compile()
}

pub(crate) enum Isa {
    Arm64,
}

struct JitCompiler<T> {
    code: Vec<u8>,
    arch: Box<dyn InlineAssembler<T>>,
}

impl<T> JitCompiler<T> {
    fn new(code: &[u8], isa: Isa) -> Self {
        match isa {
            Isa::Arm64 => JitCompiler {
                arch: Box::new(Arm64InlineAssembler::new()),
                code: code.to_vec(),
            },
        }
    }

    fn compile(&mut self) -> extern "C" fn() -> T {
        self.generate();
        self.arch.unsafe_transmute()
    }

    fn generate(&mut self) {
        todo!()
    }
}
