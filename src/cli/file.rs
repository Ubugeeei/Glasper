use crate::runtime::js::JavaScriptRuntime;

pub fn run(path: &str) {
    let mut runtime = JavaScriptRuntime::new();
    match std::fs::read_to_string(path) {
        Ok(source) => {
            let _ = runtime.execute(source);
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
