use crate::runtime::interface::JSRuntimeBuilder;

pub fn exec_source(path: &str, vm: bool) {
    let mut runtime = JSRuntimeBuilder::build(vm);
    match std::fs::read_to_string(path) {
        Ok(source) => {
            runtime.run(source);
        }
        Err(_) => {
            let crr_dir = std::env::current_dir().unwrap();
            println!(
                "\x1b[31merror\x1b[0m: Module not found \"file://{}/{}\".",
                crr_dir.display(),
                path
            );
        }
    };
}
