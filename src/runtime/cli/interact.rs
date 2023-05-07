use rustyline::error::ReadlineError;

use crate::runtime::interface::JSRuntimeBuilder;

pub fn start(vm: bool) {
    println!("Welcome to Glasper v0.1.0 ");
    println!("exit using ctrl+c or ctrl+d or exit()");

    let mut runtime = JSRuntimeBuilder::build(vm);

    let mut rl = rustyline::DefaultEditor::new().unwrap();

    loop {
        let input = rl.readline("> ");
        match input {
            Ok(line) => {
                let _ = rl.add_history_entry(&line);

                if line == "exit()" {
                    println!("Bye!");
                    break;
                }

                runtime.run(line);
            }

            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }

            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }
}
