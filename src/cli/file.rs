use crate::runtime::js::JavaScriptRuntime;

pub fn run(path: &str) {
    let mut runtime = JavaScriptRuntime::new();
    let source = std::fs::read_to_string(path).unwrap();
    let _ = runtime.execute(source);
}
