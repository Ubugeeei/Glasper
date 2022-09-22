extern crate rustyline;
use rustyline::{error::ReadlineError, Editor};

use crate::runtime::js::JavaScriptRuntime;

pub fn start() {
    println!("Welcome to Glasper v0.1.0 ");
    println!("exit using ctrl+c or ctrl+d or exit()");

    let mut runtime = JavaScriptRuntime::new();
    let mut rl = Editor::<()>::new();

    loop {
        let input = rl.readline("> ");
        match input {
            Ok(line) => {
                rl.add_history_entry(&line);

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
