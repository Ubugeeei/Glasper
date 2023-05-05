use rustyline::error::ReadlineError;

use crate::engine::execution::vm::Interpreter;
use crate::runtime::js::JavaScriptRuntime;

pub fn start(vm: bool) {
    println!("Welcome to Glasper v0.1.0 ");
    println!("exit using ctrl+c or ctrl+d or exit()");
    if vm {
        start_vm_repl();
    } else {
        start_host_repl();
    }
}

fn start_host_repl() {
    let mut runtime = JavaScriptRuntime::new();
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    loop {
        let input = rl.readline("> ");
        match input {
            Ok(line) => {
                let _ = rl.add_history_entry(&line);

                /*
                 *
                 * exit
                 *
                 */
                if line == "exit()" {
                    println!("Bye!");
                    break;
                }

                /*
                 *
                 * execute
                 *
                 */
                match runtime.execute(line) {
                    Ok(o) => println!("{}", o),
                    Err(e) => println!("{}", e),
                }
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

fn start_vm_repl() {
    let mut interpreter = Interpreter::new();
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

                if line == "%PrintDump()" {
                    interpreter.vm.print_dump();
                } else if line == "%PrintIr()" {
                    interpreter.vm.print_ir();
                } else if line == "%GetBytes()" {
                    interpreter.vm.print_bytecode();
                } else {
                    let _ = interpreter.run(line);
                    interpreter.vm.print();
                }
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
